use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct ContainerStatsOptions<'a> {
    pub containers: Option<Vec<&'a str>>,
    pub interval: Option<i64>,
    pub stream: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerStats {
    #[serde(rename = "AvgCPU")]
    pub avg_cpu: f64,
    pub block_input: u64,
    pub block_output: u64,
    #[serde(rename = "ContainerID")]
    pub container_id: String,
    #[serde(rename = "CPU")]
    pub cpu: f64,
    #[serde(rename = "CPUNano")]
    pub cpu_nano: u64,
    #[serde(rename = "CPUSystemNano")]
    pub cpu_system_nano: u64,
    pub duration: u64,
    pub mem_limit: u64,
    pub mem_perc: f64,
    pub mem_usage: u64,
    pub name: String,
    pub network: HashMap<String, ContainerStatsNetwork>,
    #[serde(rename = "PerCPU")]
    pub per_cpu: Vec<u64>,
    #[serde(rename = "PIDs")]
    pub pids: u64,
    pub system_nano: u64,
    pub up_time: u64,
}

impl fmt::Debug for ContainerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerStatsNetwork {
    pub rx_bytes: u64,
    pub rx_dropped: u64,
    pub rx_errors: u64,
    pub rx_packets: u64,
    pub tx_bytes: u64,
    pub tx_dropped: u64,
    pub tx_errors: u64,
    pub tx_packets: u64,
}
