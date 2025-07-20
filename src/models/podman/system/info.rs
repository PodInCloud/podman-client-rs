use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub host: InfoHost,
    pub store: InfoStore,
    pub registries: InfoRegistries,
    pub plugins: InfoPlugins,
    pub version: InfoVersion,
}

impl fmt::Debug for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHost {
    pub arch: String,
    pub buildah_version: String,
    pub cgroup_manager: String,
    pub cgroup_controllers: Vec<String>,
    pub conmon: InfoHostConmon,
    pub cpus: u32,
    pub cpu_utilization: InfoHostCpuUtilization,
    pub database_backend: String,
    pub distribution: InfoHostDistribution,
    pub event_logger: String,
    pub free_locks: u64,
    pub hostname: String,
    pub id_mappings: InfoHostIdMappings,
    pub kernel: String,
    pub log_driver: String,
    pub mem_free: u64,
    pub mem_total: u64,
    pub network_backend: String,
    pub network_backend_info: InfoHostNetworkBackendInfo,
    pub oci_runtime: InfoHostOciRuntime,
    pub os: String,
    pub remote_socket: InfoHostRemoteSocket,
    pub rootless_network_cmd: String,
    pub service_is_remote: bool,
    pub security: InfoHostSecurity,
    pub slirp4netns: InfoHostSlirp4netns,
    pub pasta: InfoHostPasta,
    pub swap_free: u64,
    pub swap_total: u64,
    pub uptime: String,
    pub variant: String,
    pub linkmode: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostConmon {
    pub package: String,
    pub path: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostCpuUtilization {
    pub user_percent: f32,
    pub system_percent: f32,
    pub idle_percent: f32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostDistribution {
    pub distribution: String,
    pub variant: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostIdMappings {
    pub gidmap: Vec<InfoHostIdMappingsIdMap>,
    pub uidmap: Vec<InfoHostIdMappingsIdMap>,
}

#[derive(Deserialize, Serialize)]
pub struct InfoHostIdMappingsIdMap {
    pub container_id: u32,
    pub host_id: u32,
    pub size: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostNetworkBackendInfo {
    pub backend: String,
    pub version: String,
    pub package: String,
    pub path: String,
    pub dns: InfoHostNetworkBackendInfoDns,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostNetworkBackendInfoDns {
    pub version: String,
    pub package: String,
    pub path: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostOciRuntime {
    pub name: String,
    pub package: String,
    pub path: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostRemoteSocket {
    pub path: String,
    pub exists: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostSecurity {
    pub apparmor_enabled: bool,
    pub capabilities: String,
    pub rootless: bool,
    pub seccomp_enabled: bool,
    pub seccomp_profile_path: String,
    pub selinux_enabled: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostSlirp4netns {
    pub executable: String,
    pub package: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoHostPasta {
    pub executable: String,
    pub package: String,
    pub version: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoStore {
    pub config_file: String,
    pub container_store: InfoStoreContainerStore,
    pub graph_driver_name: String,
    pub graph_options: serde_json::Value,
    pub graph_root: String,
    pub graph_root_allocated: u64,
    pub graph_root_used: u64,
    pub graph_status: InfoStoreGraphStatus,
    pub image_copy_tmp_dir: String,
    pub image_store: InfoStoreImageStore,
    pub run_root: String,
    pub volume_path: String,
    pub transient_store: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoStoreContainerStore {
    pub number: u32,
    pub paused: u32,
    pub running: u32,
    pub stopped: u32,
}

#[derive(Deserialize, Serialize)]
pub struct InfoStoreGraphStatus {
    #[serde(rename = "Backing Filesystem")]
    pub backing_filesystem: String,

    #[serde(rename = "Native Overlay Diff")]
    pub native_overlay_diff: String,

    #[serde(rename = "Supports d_type")]
    pub supports_d_type: String,

    #[serde(rename = "Supports shifting")]
    pub supports_shifting: String,

    #[serde(rename = "Supports volatile")]
    pub supports_volatile: String,

    #[serde(rename = "Using metacopy")]
    pub using_metacopy: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoStoreImageStore {
    pub number: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoRegistries {
    pub search: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoPlugins {
    pub volume: Vec<String>,
    pub network: Vec<String>,
    pub log: Vec<String>,
    pub authorization: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub struct InfoVersion {
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
