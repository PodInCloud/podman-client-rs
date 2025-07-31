use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::podman::common::{
    linux_device_cgroup::LinuxDeviceCgroup, linux_throttle_device::LinuxThrottleDevice,
    linux_weight_device::LinuxWeightDevice,
};

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResources {
    #[serde(rename = "blockIO")]
    pub block_io: Option<LinuxResourcesBlockIo>,
    pub cpu: Option<LinuxResourcesCpu>,
    pub devices: Option<Vec<LinuxDeviceCgroup>>,
    pub hugepage_limits: Option<Vec<LinuxResourcesHugepageLimit>>,
    pub memory: Option<LinuxResourcesMemory>,
    pub network: Option<LinuxResourcesNetwork>,
    pub pids: Option<LinuxResourcesPids>,
    pub rdma: Option<HashMap<String, LinuxResourcesRdma>>,
    pub unified: Option<HashMap<String, String>>,
}

impl fmt::Debug for LinuxResources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourcesBlockIo {
    pub leaf_weight: Option<u16>,
    pub throttle_read_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device: Option<Vec<LinuxThrottleDevice>>,
    pub throttle_write_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device: Option<Vec<LinuxThrottleDevice>>,
    pub weight: Option<u16>,
    #[serde(rename = "weight_device")]
    pub weight_device: Option<Vec<LinuxWeightDevice>>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourcesCpu {
    pub burst: Option<u64>,
    pub cpus: Option<String>,
    pub idle: Option<i64>,
    pub mems: Option<String>,
    pub period: Option<u64>,
    pub quota: Option<i64>,
    pub realtime_period: Option<u64>,
    pub realtime_runtime: Option<i64>,
    pub shares: Option<u64>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourcesHugepageLimit {
    pub limit: Option<u64>,
    pub page_size: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourcesMemory {
    pub check_before_update: Option<bool>,
    #[serde(rename = "disableOOMKiller")]
    pub disable_oom_killer: Option<bool>,
    pub kernel: Option<i64>,
    #[serde(rename = "kernelTCP")]
    pub kernel_tcp: Option<i64>,
    pub limit: Option<i64>,
    pub reservation: Option<i64>,
    pub swap: Option<i64>,
    pub swappiness: Option<i64>,
    pub use_hierarchy: Option<bool>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct LinuxResourcesNetwork {
    #[serde(rename = "classID")]
    pub class_id: Option<u32>,
    pub priorities: Option<Vec<LinuxResourcesNetworkPriority>>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct LinuxResourcesNetworkPriority {
    pub name: Option<String>,
    pub priority: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct LinuxResourcesPids {
    limit: i64,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourcesRdma {
    pub hca_handles: Option<u32>,
    pub hca_objects: Option<u32>,
}
