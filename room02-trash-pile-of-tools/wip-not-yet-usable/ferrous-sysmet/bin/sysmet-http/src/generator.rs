use std::{fmt::Debug, sync::Arc, time::Duration};

use log::{debug, trace, tracing};
use metrics::prelude::*;
use tokio::{
    sync::{oneshot::Receiver, RwLock},
    time::Instant,
};
use typed_builder::TypedBuilder;

use crate::{svg::values_to_polyline, ChartContext, ChartLine, ChartValue};

const ACTUALIZATION_INTERVAL: Duration = Duration::from_secs(120);

const CPU_USAGE_TITLE: &str = "CPU Usage";
const RAM_USAGE_TITLE: &str = "RAM Usage";
const LOAD_AVERAGE_TITLE: &str = "Load Average";
const NETWORK_TITLE: &str = "Network";
const DISKS_SPEED_TITLE: &str = "Disks Speed Usage";
const DISKS_MEMORY_TITLE: &str = "Disks Memory Usage";

#[derive(Debug, TypedBuilder)]
pub struct ChartsData {
    pub last_updated_time: Instant,
    pub metrics: Vec<(&'static str, ChartContext)>,
}

impl Default for ChartsData {
    fn default() -> Self {
        ChartsData {
            last_updated_time: Instant::now(),
            metrics: Vec::new(),
        }
    }
}

#[tracing::instrument(level = "debug")]
pub async fn actualization_task(
    shared_chart_data: Arc<RwLock<ChartsData>>,
    database: String,
    mut db_rx: Receiver<()>,
) {
    debug!("Spawned actualization task");
    let mut interval = tokio::time::interval(ACTUALIZATION_INTERVAL);

    loop {
        let interval = interval.tick();
        tokio::pin!(interval);

        tokio::select! {
            _ = &mut interval => {
                if let Ok(database) = Database::from_file(&database) {
                    let mut chart_data = shared_chart_data.write().await;
                    *chart_data = database.into();
                }
            }
            _ = &mut db_rx => {
                break;
            }
        }

        trace!("Looped through actualization process");
    }

    debug!("Finished actualization task");
}

impl From<Database> for ChartsData {
    fn from(chart_data: Database) -> Self {
        let snapshots_len = chart_data.snapshots.len();

        let cpus_usages: Vec<ChartValue<_>> = chart_data.get_cpu_usage().into_iter().fold(
            Vec::with_capacity(snapshots_len),
            |mut cpus_usages, (snap, timestamp)| {
                cpus_usages.push((snap, timestamp.timestamp(), ()) as ChartValue<_>);
                cpus_usages
            },
        );
        let cpu_chart = build_chart(vec![("#e00", None, cpus_usages)]);

        let (ram_usages, swap_usages): (Vec<ChartValue<_>>, Vec<ChartValue<_>>) =
            chart_data.get_ram_usage().into_iter().fold(
                (
                    Vec::with_capacity(snapshots_len),
                    Vec::with_capacity(snapshots_len),
                ),
                |(mut ram_usages, mut swap_usages), ((ram, swap), timestamp)| {
                    let time = timestamp.timestamp();
                    ram_usages.push((ram, time, ()) as ChartValue<_>);
                    swap_usages.push((swap, time, ()) as ChartValue<_>);

                    (ram_usages, swap_usages)
                },
            );
        let ram_chart = build_chart(vec![
            ("#0e0", Some("RAM"), ram_usages),
            ("#e0e", Some("Swap"), swap_usages),
        ]);

        let (load_avgs_one, load_avgs_five, load_avgs_fiveteen): (
            Vec<ChartValue<_>>,
            Vec<ChartValue<_>>,
            Vec<ChartValue<_>>,
        ) = chart_data.get_load().into_iter().fold(
            (
                Vec::with_capacity(snapshots_len),
                Vec::with_capacity(snapshots_len),
                Vec::with_capacity(snapshots_len),
            ),
            |(mut load_avgs_one, mut load_avgs_five, mut load_avgs_fiveteen),
             ((load_avg_one, load_avg_five, load_avg_fiveteen), timestamp)| {
                let time = timestamp.timestamp();
                load_avgs_one.push((load_avg_one, time, ()) as ChartValue<_>);
                load_avgs_five.push((load_avg_five, time, ()) as ChartValue<_>);
                load_avgs_fiveteen.push((load_avg_fiveteen, time, ()) as ChartValue<_>);

                (load_avgs_one, load_avgs_five, load_avgs_fiveteen)
            },
        );
        let load_avg_chart = build_chart(vec![
            ("#a0a", Some("1 minutes"), load_avgs_one),
            ("#0a0", Some("5 minutes"), load_avgs_five),
            ("#00e", Some("15 minutes"), load_avgs_fiveteen),
        ]);

        let (network_recv_usage, network_sent_usage): (Vec<ChartValue<_>>, Vec<ChartValue<_>>) =
            chart_data.get_network().into_iter().fold(
                (
                    Vec::with_capacity(snapshots_len),
                    Vec::with_capacity(snapshots_len),
                ),
                |(mut network_recv_usage, mut network_sent_usage), ((recv, sent), timestamp)| {
                    let time = timestamp.timestamp();
                    network_recv_usage.push((recv, time, ()) as ChartValue<_>);
                    network_sent_usage.push((sent, time, ()) as ChartValue<_>);

                    (network_recv_usage, network_sent_usage)
                },
            );
        let network_chart = build_chart(vec![
            ("#faa", Some("Received"), network_recv_usage),
            ("#aaf", Some("Sent"), network_sent_usage),
        ]);

        let (disk_speed_read, disk_speed_write): (Vec<ChartValue<_>>, Vec<ChartValue<_>>) =
            chart_data.get_disks_speed_usage().into_iter().fold(
                (
                    Vec::with_capacity(snapshots_len),
                    Vec::with_capacity(snapshots_len),
                ),
                |(mut disk_speed_read, mut disk_speed_write), ((read, write), timestamp)| {
                    let time = timestamp.timestamp();
                    disk_speed_read.push((read, time, ()) as ChartValue<_>);
                    disk_speed_write.push((write, time, ()) as ChartValue<_>);
                    (disk_speed_read, disk_speed_write)
                },
            );
        let disk_speed_chart = build_chart(vec![
            ("#afa", Some("Read"), disk_speed_read),
            ("#faf", Some("Write"), disk_speed_write),
        ]);

        let disk_memory_usage: Vec<ChartValue<_>> =
            chart_data.get_disk_memory_usage().into_iter().fold(
                Vec::with_capacity(snapshots_len),
                |mut disk_memory_usage, (usage, timestamp)| {
                    let time = timestamp.timestamp();
                    disk_memory_usage.push((usage, time, ()) as ChartValue<_>);
                    disk_memory_usage
                },
            );
        let disk_memory_chart = build_chart(vec![("#a4f", Some("Usage"), disk_memory_usage)]);

        let chart_sections = vec![
            (
                CPU_USAGE_TITLE,
                ChartContext::builder()
                    .max_value(cpu_chart.0)
                    .collections(cpu_chart.1)
                    .build(),
            ),
            (
                RAM_USAGE_TITLE,
                ChartContext::builder()
                    .max_value(ram_chart.0)
                    .collections(ram_chart.1)
                    .build(),
            ),
            (
                LOAD_AVERAGE_TITLE,
                ChartContext::builder()
                    .max_value(load_avg_chart.0)
                    .collections(load_avg_chart.1)
                    .build(),
            ),
            (
                NETWORK_TITLE,
                ChartContext::builder()
                    .unit("MiB") // TODO: MiB and if > 1024 GiB
                    .max_value(network_chart.0)
                    .collections(network_chart.1)
                    .build(),
            ),
            (
                DISKS_SPEED_TITLE,
                ChartContext::builder()
                    .unit("MiB")
                    .max_value(disk_speed_chart.0)
                    .collections(disk_speed_chart.1)
                    .build(),
            ),
            (
                DISKS_MEMORY_TITLE,
                ChartContext::builder()
                    .unit("MiB")
                    .max_value(disk_memory_chart.0)
                    .collections(disk_memory_chart.1)
                    .build(),
            ),
        ];

        ChartsData::builder()
            .last_updated_time(Instant::now())
            .metrics(chart_sections)
            .build()
    }
}

#[allow(clippy::type_complexity)]
fn build_chart<T: Debug>(
    collections: Vec<(&str, Option<&str>, Vec<ChartValue<T>>)>,
) -> (f64, Vec<ChartLine>) {
    let max_value = collections
        .iter()
        .flat_map(|(_, _, values)| values.iter().map(|(val, _, _)| val))
        .fold(0f64, |max, x| max.max(*x));
    trace!(max_value);
    let collections = collections
        .into_iter()
        .filter_map(|(color, label, values)| {
            values_to_polyline(&values, (0f64, max_value)).map(|polyline| {
                (
                    color.to_string(),
                    label.map(|label| label.to_string()),
                    polyline,
                )
            })
        })
        .collect::<Vec<_>>();

    (max_value, collections)
}
