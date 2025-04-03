use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use lettre::{message::Mailbox, Message};
use log::tracing;
use rust_decimal::prelude::Decimal;

use crate::{PercentSnapshot, Result};

#[tracing::instrument(level = "trace")]
pub fn format_threshold_crossed_msg<T: Debug + Display, O: Debug + Display>(
    name: &str,
    threshold: T,
    observed_value: O,
) -> Result<String> {
    Ok(format!(
        "- {name} threshold crossed ({}%): observed {}%\n",
        Decimal::from_str(&threshold.to_string())?.round_dp(3),
        Decimal::from_str(&observed_value.to_string())?.round_dp(3)
    ))
}

#[tracing::instrument(level = "debug", skip(snap))]
pub fn format_snapshot(snap: &PercentSnapshot) -> Result<String> {
    let mut body = "System state:\n".to_string();
    body.push_str(&format!(
        "- CPU {}%\n",
        Decimal::from_str(&snap.cpu.to_string())?.round_dp(3)
    ));
    body.push_str(&format!(
        "- RAM {}%\n",
        Decimal::from_str(&snap.ram.to_string())?.round_dp(3)
    ));
    body.push_str(&format!(
        "- Swap {}%\n",
        Decimal::from_str(&snap.swap.to_string())?.round_dp(3)
    ));
    body.push_str(&format!(
        "- Disk {}%\n",
        Decimal::from_str(&snap.disk.to_string())?.round_dp(3)
    ));
    body.push_str(&format!(
        "- Average Load (on 15min) {}%\n",
        Decimal::from_str(&snap.avg_load.to_string())?.round_dp(3)
    ));

    Ok(body)
}

#[tracing::instrument]
pub fn generate_mail(
    server_ident: &str,
    from: Mailbox,
    contacts: Vec<Mailbox>,
    body: &str,
) -> Result<Message> {
    let email = Message::builder()
        .date_now()
        .from(from)
        .subject(format!("Warning threshold reached on {server_ident}"));
    let email = contacts
        .into_iter()
        .fold(email, |email, contact| email.bcc(contact));

    Ok(email.body(body.to_string())?)
}
