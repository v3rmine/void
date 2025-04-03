#![forbid(unsafe_code)]

use std::{
    env::{args_os, set_var},
    ffi::OsString,
    fs::{self, File},
    io::{Seek, SeekFrom, Write},
    path::Path,
};

use clap::Parser;
pub use color_eyre::Result;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use log::{debug, error, info, trace};
use metrics::prelude::*;

use crate::mail::{format_snapshot, format_threshold_crossed_msg, generate_mail};

mod cli;
mod mail;

#[derive(Debug)]
pub struct PercentSnapshot {
    pub cpu: f32,
    pub ram: f32,
    pub swap: f32,
    pub memory: f32,
    pub disk: f32,
    pub avg_load: f32,
}

fn is_threshold_crossed(debug_msg: &str, threshold: Option<u32>, observered_value: f32) -> bool {
    let mut is_threshold_crossed = false;

    if let Some(threshold) = threshold {
        if observered_value as f64 > threshold as f64 {
            is_threshold_crossed = true;
            debug!(
                threshold = threshold,
                usage = observered_value,
                "{debug_msg}"
            );
        }
    }

    is_threshold_crossed
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut is_setup_with_specific_path = false;
    for win in args_os().collect::<Vec<OsString>>().windows(2) {
        if let Some(prev) = win.get(0) {
            if prev == "--env" {
                if let Some(next) = win.get(1) {
                    is_setup_with_specific_path = true;
                    env::setup_env_with_path(Path::new(next));
                }
            }
        }
    }
    if !is_setup_with_specific_path {
        // Setup env to .env
        env::setup_env();
    }

    let app = cli::Cli::parse();
    if app.verbose.is_silent() {
        set_var("LOG_LEVEL", "SILENT");
    } else if let Some(level) = app.verbose.log_level() {
        set_var("LOG_LEVEL", level.as_str());
    }

    log::setup_simple_logger();
    let hostname = get_hostname();
    info!("Check started on device {hostname}");
    trace!(args =? app, "Cli called with args on device {hostname}");

    let now = chrono::Utc::now();
    if !app.dry_run {
        if let Some(path) = &app.last_sent_instant {
            let content = fs::read_to_string(path).unwrap_or_default();
            let after_cooldown = if !content.is_empty() {
                match content.parse::<chrono::DateTime<chrono::Utc>>() {
                    Ok(date) => date
                        .checked_add_signed(chrono::Duration::from_std(app.cooldown)?)
                        .map_or(true, |i| i < now),
                    Err(_) => true,
                }
            } else {
                true
            };

            if !after_cooldown {
                info!("No need to take check usages, we are before the end of the cooldown");
                return Ok(());
            }
        }
    }

    let pretty_formated_now = now.format("%d/%m/%Y %H:%M");

    let (ram, swap) = memory_usage_percent()?;
    let snapshot = PercentSnapshot {
        cpu: cpu_usage_percent()?,
        ram,
        swap,
        memory: (ram + swap) / 2.0, // REVIEW: Might need a more precise way of calculating average memory load
        disk: disk_usage_percent()?,
        avg_load: load_avg_percent()?.2,
    };

    trace!(snapshot =? snapshot, "System snapshot taken at {pretty_formated_now}");

    let cpu_threshold_crossed =
        is_threshold_crossed("CPU threshold crossed", app.cpu_threshold, snapshot.cpu);
    let ram_threshold_crossed =
        is_threshold_crossed("RAM threshold crossed", app.ram_threshold, snapshot.ram);
    let swap_threshold_crossed =
        is_threshold_crossed("Swap threshold crossed", app.swap_threshold, snapshot.swap);
    let memory_threshold_crossed = is_threshold_crossed(
        "Memory threshold crossed",
        app.memory_threshold,
        snapshot.memory,
    );
    let disk_threshold_crossed =
        is_threshold_crossed("Disk threshold crossed", app.disk_threshold, snapshot.disk);
    let avg_load_threshold_crossed = is_threshold_crossed(
        "Average Load threshold crossed",
        app.avg_load_threshold,
        snapshot.avg_load,
    );
    let mut body = "Thresholds crossed:\n".to_string();

    let mut at_least_one_threshold_crossed = false;
    if cpu_threshold_crossed {
        at_least_one_threshold_crossed = true;
        body.push_str(
            &(format_threshold_crossed_msg("CPU", app.cpu_threshold.unwrap(), snapshot.cpu)?),
        );
    }
    if ram_threshold_crossed {
        at_least_one_threshold_crossed = true;
        body.push_str(
            &(format_threshold_crossed_msg("RAM", app.ram_threshold.unwrap(), snapshot.ram)?),
        );
    }
    if swap_threshold_crossed {
        at_least_one_threshold_crossed = true;
        body.push_str(
            &(format_threshold_crossed_msg("Swap", app.swap_threshold.unwrap(), snapshot.swap)?),
        );
    }
    if memory_threshold_crossed {
        at_least_one_threshold_crossed = true;
        body.push_str(
            &(format_threshold_crossed_msg(
                "RAM & Swap",
                app.memory_threshold.unwrap(),
                snapshot.memory,
            )?),
        );
    }
    if disk_threshold_crossed {
        at_least_one_threshold_crossed = true;
        body.push_str(
            &(format_threshold_crossed_msg("Disk", app.disk_threshold.unwrap(), snapshot.disk)?),
        );
    }
    if avg_load_threshold_crossed {
        at_least_one_threshold_crossed = true;
        body.push_str(
            &(format_threshold_crossed_msg(
                "Average Load",
                app.avg_load_threshold.unwrap(),
                snapshot.avg_load,
            )?),
        );
    }

    if !at_least_one_threshold_crossed {
        info!("Finishing early because no threshold have been crossed");
        return Ok(()); // Exit SUCCESS;
    } else {
        info!("At least one threshold crossed!");
    }

    body.push_str("\n\n");
    body.push_str(&format_snapshot(&snapshot)?);

    debug!(body, "Body that will be sent");

    if app.dry_run {
        info!(
            "Finishing early because there is no need to send a mail, the app is in dry-run mode"
        );
        return Ok(()); // Exit SUCCESS;
    }

    let smtp_relay = app.smtp_relay.unwrap();
    let smtp_user = app.smtp_user.unwrap();
    let smtp_password = app.smtp_password.unwrap();
    let last_sent_instant = app.last_sent_instant.unwrap();

    let email = generate_mail(
        &hostname,
        app.from.unwrap_or("user@example.org".parse()?),
        app.contacts,
        &body,
    )?;

    let mailer = SmtpTransport::relay(&smtp_relay)?
        .port(app.smtp_port)
        .credentials(Credentials::new(smtp_user, smtp_password))
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            info!("Mail sent successfully!");

            let mut last_mail_instant = File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(last_sent_instant)?;
            last_mail_instant.seek(SeekFrom::Start(0))?;
            last_mail_instant.write_all(now.to_rfc3339().as_bytes())?;
        }
        Err(e) => error!(error =? e, "Failed to send mail because an error happened"),
    }

    Ok(())
}
