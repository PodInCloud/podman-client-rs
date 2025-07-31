use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::podman::common::{
    id_mapping_options::IdMappingOptions, image_volume::ImageVolume,
    linux_device_cgroup::LinuxDeviceCgroup, linux_resources::LinuxResources,
    linux_throttle_device::LinuxThrottleDevice, linux_weight_device::LinuxWeightDevice,
    named_volume::NamedVolume, namespace::Namespace, overlay_volume::OverlayVolume,
    per_network_options::PerNetworkOptions, port_mapping::PortMapping,
    schema2_health_config::Schema2HealthConfig, volume_mount::VolumeMount,
};

#[derive(Serialize, Default)]
pub struct ContainerCreateOptions {
    pub annotations: Option<HashMap<String, String>>,
    pub apparmor_profile: Option<String>,
    pub artifact_volumes: Option<Vec<ContainerCreateArtifactVolumeOptions>>,
    pub base_hosts_file: Option<String>,
    pub cap_add: Option<Vec<String>>,
    pub cap_drop: Option<Vec<String>>,
    pub cgroup_parent: Option<String>,
    pub cgroupns: Option<Namespace>,
    pub cgroups_mode: Option<String>,
    pub chroot_directories: Option<Vec<String>>,
    pub cni_networks: Option<Vec<String>>,
    pub command: Option<Vec<String>>,
    pub conmon_pid_file: Option<String>,
    #[serde(rename = "containerCreateCommand")]
    pub container_create_command: Option<Vec<String>>,
    pub create_working_dir: Option<bool>,
    #[serde(rename = "dependencyContainers")]
    pub dependency_containers: Option<Vec<String>>,
    pub device_cgroup_rule: Option<Vec<LinuxDeviceCgroup>>,
    pub devices: Option<Vec<ContainerCreateLinuxDeviceOptions>>,
    pub devices_from: Option<Vec<String>>,
    pub dns_option: Option<Vec<String>>,
    pub dns_search: Option<Vec<String>>,
    pub dns_server: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub env_host: Option<bool>,
    pub envmerge: Option<Vec<String>>,
    pub expose: Option<HashMap<u16, String>>,
    pub group_entry: Option<String>,
    pub groups: Option<Vec<String>>,
    pub health_check_on_failure_action: Option<i64>,
    pub healthconfig: Option<Schema2HealthConfig>,
    #[serde(rename = "healthLogDestination")]
    pub health_log_destination: Option<String>,
    #[serde(rename = "healthMaxLogCount")]
    pub health_max_log_count: Option<u64>,
    #[serde(rename = "healthMaxLogSize")]
    pub health_max_log_size: Option<u64>,
    pub host_device_list: Option<Vec<ContainerCreateLinuxDeviceOptions>>,
    pub hostadd: Option<Vec<String>>,
    pub hostname: Option<String>,
    pub hostusers: Option<Vec<String>>,
    pub httpproxy: Option<bool>,
    pub idmappings: Option<IdMappingOptions>,
    pub image: Option<String>,
    pub image_arch: Option<String>,
    pub image_os: Option<String>,
    pub image_variant: Option<String>,
    pub image_volume_mode: Option<String>,
    pub image_volumes: Option<Vec<ImageVolume>>,
    pub init: Option<bool>,
    pub init_container_type: Option<String>,
    pub init_path: Option<String>,
    #[serde(rename = "intelRdt")]
    pub intel_rdt: Option<ContainerCreateIntelRdtOptions>,
    pub ipcns: Option<Namespace>,
    pub label_nested: Option<bool>,
    pub labels: Option<HashMap<String, String>>,
    pub log_configuration: Option<ContainerCreateLogConfigurationOptions>,
    pub manage_password: Option<bool>,
    pub mask: Option<Vec<String>>,
    pub mounts: Option<Vec<VolumeMount>>,
    pub name: Option<String>,
    pub netns: Option<Namespace>,
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "Networks")]
    pub networks: Option<HashMap<String, PerNetworkOptions>>,
    pub no_new_privileges: Option<bool>,
    pub oci_runtime: Option<String>,
    pub oom_score_adj: Option<i64>,
    pub overlay_volumes: Option<Vec<OverlayVolume>>,
    pub passwd_entry: Option<String>,
    pub personality: Option<ContainerCreatePersonalityOptions>,
    pub pidns: Option<Namespace>,
    pub pod: Option<String>,
    pub portmappings: Option<Vec<PortMapping>>,
    pub privileged: Option<bool>,
    pub procfs_opts: Option<Vec<String>>,
    pub publish_image_ports: Option<bool>,
    pub r_limits: Option<Vec<ContainerCreateRLimitOptions>>,
    pub raw_image_name: Option<String>,
    pub read_only_filesystem: Option<bool>,
    pub read_write_tmpfs: Option<bool>,
    pub remove: Option<bool>,
    #[serde(rename = "removeImage")]
    pub remove_image: Option<bool>,
    pub resource_limits: Option<LinuxResources>,
    pub restart_policy: Option<String>,
    pub restart_tries: Option<u64>,
    pub rootfs: Option<String>,
    pub rootfs_mapping: Option<String>,
    pub rootfs_overlay: Option<bool>,
    pub rootfs_propagation: Option<String>,
    #[serde(rename = "sdnotifyMode")]
    pub sdnotify_mode: Option<String>,
    pub seccomp_policy: Option<String>,
    pub seccomp_profile_path: Option<String>,
    pub secret_env: Option<HashMap<String, String>>,
    pub secrets: Option<Vec<ContainerCreateSecretOptions>>,
    pub selinux_opts: Option<Vec<String>>,
    pub shm_size: Option<i64>,
    pub shm_size_systemd: Option<i64>,
    #[serde(rename = "startupHealthConfig")]
    pub startup_health_config: Option<ContainerCreateStartupHealthConfigOptions>,
    pub stdin: Option<bool>,
    pub stop_signal: Option<i64>,
    pub stop_timeout: Option<u64>,
    pub storage_opts: Option<HashMap<String, String>>,
    pub sysctl: Option<HashMap<String, String>>,
    pub systemd: Option<String>,
    pub terminal: Option<bool>,
    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteBpsDevice")]
    pub throttle_write_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device: Option<HashMap<String, LinuxThrottleDevice>>,
    pub timeout: Option<u64>,
    pub timezone: Option<String>,
    pub umask: Option<String>,
    pub unified: Option<HashMap<String, String>>,
    pub unmask: Option<Vec<String>>,
    pub unsetenv: Option<Vec<String>>,
    pub unsetenvall: Option<bool>,
    pub use_image_hostname: Option<bool>,
    pub use_image_hosts: Option<bool>,
    pub use_image_resolve_conf: Option<bool>,
    pub user: Option<String>,
    pub userns: Option<Namespace>,
    pub utsns: Option<Namespace>,
    pub volatile: Option<bool>,
    pub volumes: Option<Vec<NamedVolume>>,
    pub volumes_from: Option<Vec<String>>,
    pub weight_device: Option<HashMap<String, LinuxWeightDevice>>,
    pub work_dir: Option<String>,
}

#[derive(Serialize, Default)]
pub struct ContainerCreateArtifactVolumeOptions {
    pub destination: Option<String>,
    pub digest: Option<String>,
    pub name: Option<String>,
    pub source: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerCreateLinuxDeviceOptions {
    pub file_mode: Option<u32>,
    pub gid: Option<u32>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub path: Option<String>,
    pub r#type: Option<String>,
    pub uid: Option<u32>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerCreateIntelRdtOptions {
    #[serde(rename = "closID")]
    pub clos_id: Option<String>,
    #[serde(rename = "enableCMT")]
    pub enable_cmt: Option<bool>,
    #[serde(rename = "enableMBM")]
    pub enable_mbm: Option<bool>,
    pub l3_cache_schema: Option<String>,
    pub mem_bw_schema: Option<String>,
}

#[derive(Serialize, Default)]
pub struct ContainerCreateLogConfigurationOptions {
    pub driver: Option<String>,
    pub options: Option<HashMap<String, String>>,
    pub path: Option<String>,
    pub size: Option<i64>,
}

#[derive(Serialize, Default)]
pub struct ContainerCreatePersonalityOptions {
    pub domain: Option<String>,
    pub flags: Option<Vec<String>>,
}

#[derive(Serialize, Default)]
pub struct ContainerCreateRLimitOptions {
    pub hard: Option<u64>,
    pub soft: Option<u64>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreateSecretOptions {
    pub key: Option<String>,
    pub secret: Option<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreateStartupHealthConfigOptions {
    pub interval: Option<i64>,
    pub retries: Option<i64>,
    pub start_interval: Option<i64>,
    pub start_period: Option<i64>,
    pub successes: Option<i64>,
    pub test: Option<Vec<String>>,
    pub timeout: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct ContainerCreate {
    pub id: String,
    pub warnings: Vec<String>,
}

impl fmt::Debug for ContainerCreate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
