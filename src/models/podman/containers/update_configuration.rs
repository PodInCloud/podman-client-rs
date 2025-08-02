use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::podman::common::{
    blkio_weight_device::BlkioWeightDevice,
    linux_device_cgroup::LinuxDeviceCgroup,
    linux_resources::{
        LinuxResourcesBlockIo, LinuxResourcesCpu, LinuxResourcesHugepageLimit,
        LinuxResourcesMemory, LinuxResourcesNetwork, LinuxResourcesPids, LinuxResourcesRdma,
    },
};

#[derive(Default)]
pub struct ContainerUpdateConfigurationOptions<'a> {
    pub name: &'a str,
    pub restart_policy: Option<&'a str>,
    pub restart_retries: Option<i64>,
    pub request: Option<ContainerUpdateConfigurationRequest>,
}

#[derive(Serialize, Default)]
pub struct ContainerUpdateConfigurationRequest {
    #[serde(rename = "BlkIOWeightDevice")]
    pub blkio_weight_device: Option<Vec<BlkioWeightDevice>>,
    #[serde(rename = "blockIO")]
    pub block_io: Option<LinuxResourcesBlockIo>,
    pub cpu: Option<LinuxResourcesCpu>,
    #[serde(rename = "DeviceReadBPs")]
    pub device_read_bps: Option<Vec<ContainerUpdateConfigurationRequestThrottleDevice>>,
    #[serde(rename = "DeviceReadIOPs")]
    pub device_read_iops: Option<Vec<ContainerUpdateConfigurationRequestThrottleDevice>>,
    pub devices: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(rename = "DeviceWriteBPs")]
    pub device_write_bps: Option<Vec<ContainerUpdateConfigurationRequestThrottleDevice>>,
    #[serde(rename = "DeviceWriteIOPs")]
    pub device_write_iops: Option<Vec<ContainerUpdateConfigurationRequestThrottleDevice>>,
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,
    pub health_cmd: Option<String>,
    pub health_interval: Option<String>,
    pub health_log_destination: Option<String>,
    pub health_max_log_count: Option<u64>,
    pub health_max_log_size: Option<u64>,
    pub health_on_failure: Option<String>,
    pub health_retries: Option<u64>,
    pub health_start_period: Option<String>,
    pub health_startup_cmd: Option<String>,
    pub health_startup_interval: Option<String>,
    pub health_startup_retries: Option<u64>,
    pub health_startup_success: Option<u64>,
    pub health_startup_timeout: Option<String>,
    pub health_timeout: Option<String>,
    #[serde(rename = "hugepageLimits")]
    pub hugepage_limits: Option<Vec<LinuxResourcesHugepageLimit>>,
    pub memory: Option<LinuxResourcesMemory>,
    pub network: Option<LinuxResourcesNetwork>,
    pub no_healthcheck: Option<bool>,
    pub pids: Option<LinuxResourcesPids>,
    pub rdma: Option<HashMap<String, LinuxResourcesRdma>>,
    pub unified: Option<HashMap<String, String>>,
    #[serde(rename = "UnsetEnv")]
    pub unset_env: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct ContainerUpdateConfigurationRequestThrottleDevice {
    pub path: String,
    pub rate: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ContainerUpdateConfiguration {
    pub id: String,
}
