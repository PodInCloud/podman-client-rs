use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub host: SystemInfoHost,
    pub store: SystemInfoStore,
    pub registries: HashMap<String, serde_json::Value>,
    pub plugins: SystemInfoPlugins,
    pub version: SystemInfoVersion,
}

impl fmt::Debug for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHost {
    pub arch: String,
    pub buildah_version: String,
    pub cgroup_manager: String,
    pub cgroup_controllers: Vec<String>,
    pub conmon: SystemInfoHostConmon,
    pub cpus: u32,
    pub cpu_utilization: SystemInfoHostCpuUtilization,
    pub database_backend: String,
    pub distribution: SystemInfoHostDistribution,
    pub event_logger: String,
    pub free_locks: u64,
    pub hostname: String,
    pub id_mappings: SystemInfoHostIdMappings,
    pub kernel: String,
    pub log_driver: String,
    pub mem_free: u64,
    pub mem_total: u64,
    pub network_backend: String,
    pub network_backend_info: SystemInfoHostNetworkBackendSystemInfo,
    pub oci_runtime: SystemInfoHostOciRuntime,
    pub os: String,
    pub remote_socket: SystemInfoHostRemoteSocket,
    pub rootless_network_cmd: String,
    pub service_is_remote: bool,
    pub security: SystemInfoHostSecurity,
    pub slirp4netns: SystemInfoHostSlirp4netns,
    pub pasta: SystemInfoHostPasta,
    pub swap_free: u64,
    pub swap_total: u64,
    pub uptime: String,
    pub variant: String,
    pub linkmode: String,
    pub emulated_architectures: Option<Vec<String>>,
    pub runtime_info: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostConmon {
    pub package: String,
    pub path: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostCpuUtilization {
    pub user_percent: f32,
    pub system_percent: f32,
    pub idle_percent: f32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostDistribution {
    pub distribution: String,
    pub variant: String,
    pub version: String,
    pub codename: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostIdMappings {
    pub gidmap: Vec<SystemInfoHostIdMappingsIdMap>,
    pub uidmap: Vec<SystemInfoHostIdMappingsIdMap>,
}

#[derive(Deserialize, Serialize)]
pub struct SystemInfoHostIdMappingsIdMap {
    pub container_id: u32,
    pub host_id: u32,
    pub size: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostNetworkBackendSystemInfo {
    pub backend: String,
    pub version: String,
    pub package: String,
    pub path: String,
    pub dns: SystemInfoHostNetworkBackendSystemInfoDns,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostNetworkBackendSystemInfoDns {
    pub version: String,
    pub package: String,
    pub path: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostOciRuntime {
    pub name: String,
    pub package: String,
    pub path: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostRemoteSocket {
    pub path: String,
    pub exists: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostSecurity {
    pub apparmor_enabled: bool,
    pub capabilities: String,
    pub rootless: bool,
    pub seccomp_enabled: bool,
    pub seccomp_profile_path: String,
    pub selinux_enabled: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostSlirp4netns {
    pub executable: String,
    pub package: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoHostPasta {
    pub executable: String,
    pub package: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoStore {
    pub config_file: String,
    pub container_store: SystemInfoStoreContainerStore,
    pub graph_driver_name: String,
    pub graph_options: HashMap<String, serde_json::Value>,
    pub graph_root: String,
    pub graph_root_allocated: u64,
    pub graph_root_used: u64,
    pub graph_status: HashMap<String, String>,
    pub image_copy_tmp_dir: String,
    pub image_store: SystemInfoStoreImageStore,
    pub run_root: String,
    pub volume_path: String,
    pub transient_store: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoStoreContainerStore {
    pub number: u32,
    pub paused: u32,
    pub running: u32,
    pub stopped: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoStoreImageStore {
    pub number: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoPlugins {
    pub volume: Vec<String>,
    pub network: Vec<String>,
    pub log: Vec<String>,
    pub authorization: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct SystemInfoVersion {
    #[serde(rename = "APIVersion")]
    pub api_version: String,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "GoVersion")]
    pub go_version: String,

    #[serde(rename = "GitCommit")]
    pub git_commit: String,

    #[serde(rename = "BuiltTime")]
    pub built_time: String,

    #[serde(rename = "Built")]
    pub built: u64,

    #[serde(rename = "BuildOrigin")]
    pub build_origin: String,

    #[serde(rename = "OsArch")]
    pub os_arch: String,

    #[serde(rename = "Os")]
    pub os: String,
}
