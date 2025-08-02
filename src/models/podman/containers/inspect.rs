use core::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::podman::common::{
    blkio_weight_device::BlkioWeightDevice, health_check::HealthCheck,
    inspect_device::InspectDevice, inspect_host_port::InspectHostPort, inspect_mount::InspectMount,
    schema2_health_config::Schema2HealthConfig,
};

#[derive(Default)]
pub struct ContainerInspectOptions<'a> {
    pub name: &'a str,
    pub size: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspect {
    pub app_armor_profile: String,
    pub args: Vec<String>,
    pub bounding_caps: Vec<String>,
    pub config: ContainerInspectConfig,
    pub conmon_pid_file: String,
    pub created: DateTime<Utc>,
    pub dependencies: Vec<String>,
    pub driver: String,
    pub effective_caps: Vec<String>,
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Vec<String>,
    pub graph_driver: ContainerInspectGraphDriver,
    pub host_config: ContainerInspectHostConfig,
    pub hostname_path: String,
    pub hosts_path: String,
    pub id: String,
    pub image: String,
    pub image_digest: String,
    pub image_name: String,
    pub is_infra: bool,
    pub is_service: bool,
    pub kube_exit_code_propagation: String,
    #[serde(rename = "lockNumber")]
    pub lock_number: u32,
    pub mount_label: String,
    pub mounts: Vec<InspectMount>,
    pub name: String,
    pub namespace: String,
    pub network_settings: ContainerInspectNetworkSettings,
    #[serde(rename = "OCIConfigPath")]
    pub oci_config_path: String,
    #[serde(rename = "OCIRuntime")]
    pub oci_runtime: String,
    pub path: String,
    pub pid_file: String,
    pub pod: String,
    pub process_label: String,
    pub resolv_conf_path: String,
    pub restart_count: i32,
    pub rootfs: String,
    pub size_root_fs: i64,
    pub size_rw: i64,
    pub state: ContainerInspectState,
    pub static_dir: String,
    pub use_image_hostname: bool,
    pub use_image_hosts: bool,
}

impl fmt::Debug for ContainerInspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectConfig {
    pub annotations: HashMap<String, String>,
    pub attach_stderr: bool,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub chroot_dirs: Vec<String>,
    pub cmd: Vec<String>,
    pub create_command: Vec<String>,
    pub domainname: String,
    pub entrypoint: Vec<String>,
    pub env: Vec<String>,
    pub exposed_ports: HashMap<String, serde_json::Value>,
    pub healthcheck: Schema2HealthConfig,
    pub healthcheck_max_log_count: u64,
    pub healthcheck_max_log_size: u64,
    pub healthcheck_on_failure_action: String,
    pub health_log_destination: String,
    pub hostname: String,
    pub image: String,
    pub labels: HashMap<String, String>,
    pub on_build: String,
    pub open_stdin: bool,
    pub passwd: bool,
    #[serde(rename = "sdNotifyMode")]
    pub sd_notify_mode: String,
    #[serde(rename = "sdNotifySocket")]
    pub sd_notify_socket: String,
    pub secrets: Vec<ContainerInspectConfigSecret>,
    pub startup_health_check: ContainerInspectConfigStartupHealthCheck,
    pub stdin_once: bool,
    pub stop_signal: String,
    pub stop_timeout: u64,
    pub systemd_mode: bool,
    pub timeout: u64,
    pub timezone: String,
    pub tty: bool,
    pub umask: String,
    pub user: String,
    pub volumes: HashMap<String, serde_json::Value>,
    pub working_dir: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectConfigSecret {
    #[serde(rename = "GID")]
    pub gid: u32,
    #[serde(rename = "ID")]
    pub id: String,
    pub mode: u32,
    pub name: String,
    #[serde(rename = "UID")]
    pub uid: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectConfigStartupHealthCheck {
    pub interval: Option<i64>,
    pub retries: Option<i64>,
    pub start_interval: Option<i64>,
    pub start_period: Option<i64>,
    pub successes: Option<i64>,
    pub test: Option<Vec<String>>,
    pub timeout: Option<i64>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectGraphDriver {
    pub data: HashMap<String, String>,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectHostConfig {
    pub annotations: HashMap<String, String>,
    pub auto_remove: bool,
    pub auto_remove_image: bool,
    pub binds: Vec<String>,
    pub blkio_device_read_bps: Vec<ContainerInspectHostConfigBlkioThrottleDevice>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Vec<ContainerInspectHostConfigBlkioThrottleDevice>,
    pub blkio_device_write_bps: Vec<ContainerInspectHostConfigBlkioThrottleDevice>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Vec<ContainerInspectHostConfigBlkioThrottleDevice>,
    pub blkio_weight: u16,
    pub blkio_weight_device: Vec<BlkioWeightDevice>,
    pub cap_add: Vec<String>,
    pub cap_drop: Vec<String>,
    pub cgroup: String,
    pub cgroup_conf: HashMap<String, String>,
    pub cgroup_manager: String,
    pub cgroup_mode: String,
    pub cgroup_parent: String,
    pub cgroups: String,
    pub console_size: Vec<u64>,
    #[serde(rename = "ContainerIDFile")]
    pub container_id_file: String,
    pub cpu_count: u64,
    pub cpu_percent: u64,
    pub cpu_period: u64,
    pub cpu_quota: i64,
    pub cpu_realtime_period: u64,
    pub cpu_realtime_runtime: i64,
    pub cpuset_cpus: String,
    pub cpuset_mems: String,
    pub cpu_shares: u64,
    pub devices: Vec<InspectDevice>,
    pub disk_quota: u64,
    pub dns: Vec<String>,
    pub dns_options: Vec<String>,
    pub dns_search: Vec<String>,
    pub extra_hosts: Vec<String>,
    pub group_add: Vec<String>,
    pub hosts_file: String,
    #[serde(rename = "IDMappings")]
    pub id_mappings: ContainerInspectHostConfigIdMappings,
    pub init: bool,
    #[serde(rename = "IntelRdtClosID")]
    pub intel_rdt_clos_id: String,
    #[serde(rename = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: u64,
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_iops: u64,
    pub ipc_mode: String,
    pub isolation: String,
    pub kernel_memory: i64,
    pub links: Vec<String>,
    pub log_config: ContainerInspectHostConfigLogConfig,
    pub memory: i64,
    pub memory_reservation: i64,
    pub memory_swap: i64,
    pub memory_swappiness: i64,
    pub nano_cpus: i64,
    pub network_mode: String,
    pub oom_kill_disable: bool,
    pub oom_score_adj: i64,
    pub pid_mode: String,
    pub pids_limit: i64,
    pub port_bindings: HashMap<String, Vec<InspectHostPort>>,
    pub privileged: bool,
    pub publish_all_ports: bool,
    pub readonly_rootfs: bool,
    pub restart_policy: ContainerInspectHostConfigRestartPolicy,
    pub runtime: String,
    pub security_opt: Vec<String>,
    pub shm_size: i64,
    pub tmp_fs: HashMap<String, String>,
    pub ulimits: Vec<ContainerInspectHostConfigUlimit>,
    pub userns_mode: String,
    #[serde(rename = "UTSMode")]
    pub uts_mode: String,
    pub volume_driver: String,
    pub volumes_from: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectHostConfigBlkioThrottleDevice {
    pub path: String,
    pub rate: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectHostConfigIdMappings {
    pub gid_map: Vec<String>,
    pub uid_map: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectHostConfigLogConfig {
    pub config: HashMap<String, String>,
    pub path: String,
    pub size: String,
    pub tag: String,
    pub r#type: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectHostConfigRestartPolicy {
    pub maximum_retry_count: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectHostConfigUlimit {
    pub hard: i64,
    pub name: String,
    pub soft: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectNetworkSettings {
    #[serde(rename = "AdditionalMACAddresses")]
    pub additional_mac_addresses: Vec<String>,
    pub bridge: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipb6_prefix_len: i64,
    pub hairpin_mode: bool,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_ipv6_address: String,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6_prefix_len: i64,
    pub mac_address: String,
    pub networks: HashMap<String, ContainerInspectNetworkSettingsNetwork>,
    pub ports: HashMap<String, Vec<InspectHostPort>>,
    #[serde(rename = "SandboxID")]
    pub sandbox_id: String,
    pub sandbox_key: String,
    pub secondary_ip_addresses: Vec<ContainerInspectNetworkSettingsSecondaryIPAddress>,
    pub secondary_ipv6_addresses: Vec<ContainerInspectNetworkSettingsSecondaryIPAddress>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectNetworkSettingsNetwork {
    #[serde(rename = "AdditionalMACAddresses")]
    pub additional_mac_addresses: Vec<String>,
    pub aliases: Vec<String>,
    pub driver_opts: HashMap<String, String>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: i64,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPAMConfig")]
    pub ipam_config: HashMap<String, String>,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    pub links: Vec<String>,
    pub mac_address: String,
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ip_addresses: Vec<ContainerInspectNetworkSettingsSecondaryIPAddress>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    pub secondary_ipv6_addresses: Vec<ContainerInspectNetworkSettingsSecondaryIPAddress>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectNetworkSettingsSecondaryIPAddress {
    pub addr: String,
    pub prefix_length: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectState {
    pub cgroup_path: String,
    pub checkpointed: bool,
    pub checkpointed_at: DateTime<Utc>,
    pub checkpoint_log: String,
    pub checkpoint_path: String,
    pub conmon_pid: i64,
    pub dead: bool,
    pub error: String,
    pub exit_code: i32,
    pub finished_at: DateTime<Utc>,
    pub health: HealthCheck,
    pub oci_version: String,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    pub paused: bool,
    pub pid: i64,
    pub restarting: bool,
    pub restored: bool,
    pub restored_at: DateTime<Utc>,
    pub restore_log: String,
    pub running: bool,
    pub started_at: DateTime<Utc>,
    pub status: String,
    pub stopped_by_user: bool,
}
