use std::collections::HashMap;

use ::psutil::{
    cpu::{cpu_times_percpu, CpuTimes},
    disk::{disk_usage, partitions_physical, DiskIoCounters, DiskIoCountersCollector},
    memory::{swap_memory, virtual_memory, SwapMemory, VirtualMemory},
    network::{NetIoCounters, NetIoCountersCollector},
    sensors::{temperatures, TemperatureSensor},
};
use chrono::{DateTime, Utc};
use log::{debug, tracing};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SnapShot {
    pub cpus: Vec<CpuTimes>,
    pub memory: VirtualMemory,
    pub swap: SwapMemory,
    pub networks: Vec<NetIoCounters>,
    pub disks_io: HashMap<String, DiskIoCounters>,
    pub disks_memory: HashMap<String, f32>,
    pub temps: Vec<TemperatureSensor>,
    pub load_avgs: crate::psutil::LoadAvg,
    pub time: DateTime<Utc>,
}

impl SnapShot {
    #[tracing::instrument]
    pub fn new(networks_to_ignore: &[&str]) -> Result<Self> {
        let result = Self {
            cpus: cpu_times_percpu()?,
            memory: virtual_memory()?,
            swap: swap_memory()?,
            networks: NetIoCountersCollector::default()
                .net_io_counters_pernic()?
                .into_iter()
                .filter_map(|(k, v)| {
                    if networks_to_ignore.contains(&k.as_str()) {
                        None
                    } else {
                        Some(v)
                    }
                })
                .collect(),
            disks_io: DiskIoCountersCollector::default().disk_io_counters_per_partition()?,
            disks_memory: partitions_physical()?
                .iter()
                .map(|part| -> crate::Result<(String, f32)> {
                    Ok((
                        part.mountpoint().to_string_lossy().to_string(),
                        disk_usage(part.mountpoint())?.percent(),
                    ))
                })
                .collect::<std::result::Result<HashMap<_, _>, _>>()?,
            temps: temperatures()
                .into_iter()
                .collect::<std::result::Result<Vec<TemperatureSensor>, _>>()?,
            load_avgs: crate::psutil::LoadAvg::new()?,
            time: Utc::now(),
        };

        log::trace!("Snapshot taken with data\n{:#?}", result);

        Ok(result)
    }

    #[tracing::instrument(skip(self))]
    pub fn get_cpu_count(&self) -> usize {
        self.cpus.len()
    }

    #[tracing::instrument(skip(self))]
    pub fn get_cpu_time(&self) -> (f64, f64) {
        let result = self.cpus.iter().fold((0.0, 0.0), |(busy, total), cpu| {
            (
                busy + cpu.busy().as_secs_f64(),
                total + cpu.total().as_secs_f64(),
            )
        });
        debug!(active_time = result.0, total_time = result.1);
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_ram_usage(&self) -> (f64, f64) {
        let result = (self.memory.percent() as f64, self.swap.percent() as f64);
        debug!(
            ram_percent_usage = result.0,
            ram_total = self.memory.total(),
            swap_percent_usage = result.1,
            swap_total = self.swap.total()
        );
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_load(&self) -> (f64, f64, f64) {
        let result = (
            self.load_avgs.one,
            self.load_avgs.five,
            self.load_avgs.fifteen,
        );
        debug!(
            load_1_min = result.0,
            load_5_min = result.1,
            load_15_min = result.2
        );
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_network_usage(&self) -> (f64, f64) {
        let result = self.networks.iter().fold((0.0, 0.0), |(rx, tx), net| {
            (rx + net.bytes_recv() as f64, tx + net.bytes_sent() as f64)
        });
        debug!(bytes_received = result.0, bytes_sents = result.1);
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_disk_speed_usage(&self) -> (u64, u64) {
        let result = self
            .disks_io
            .iter()
            .fold((0, 0), |(read, written), (_, disk)| {
                (read + disk.read_bytes(), written + disk.write_bytes())
            });
        debug!(bytes_rode = result.0, bytes_written = result.1);
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_disks_size_usage(&self) -> Vec<(String, f64)> {
        let result = self
            .disks_memory
            .iter()
            .map(|(name, usage)| (name.clone(), *usage as f64))
            .collect();
        debug!(disks_size_usage = ?result);
        result
    }

    pub fn try_default() -> Result<Self> {
        Self::new(&[])
    }
}
