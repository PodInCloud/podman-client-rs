use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::podman::common::id_map::IdMap;

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreateOptions {
    pub cgroup_parent: Option<String>,
    pub cni_networks: Option<String>,
    pub dns_option: Option<Vec<String>>,
    pub dns_search: Option<Vec<String>>,
    pub dns_server: Option<Vec<String>>,
    pub exit_policy: Option<String>,
    pub hostadd: Option<Vec<String>>,
    pub hostname: Option<String>,
    #[serde(rename = "hostsFile")]
    pub hosts_file: Option<String>,
    pub idmappings: Option<PodCreateIdmappingsOptions>,
    pub image_volumes: Option<Vec<PodCreateImageVolumeItemOptions>>,
    pub infra_command: Option<Vec<String>>,
    pub infra_conmon_pid_file: Option<String>,
    pub infra_image: Option<String>,
    pub infra_name: Option<String>,
    pub ipcns: Option<PodCreateNamespace>,
    pub labels: Option<HashMap<String, String>>,
    pub mounts: Option<Vec<PodCreateMountsOptions>>,
    pub name: Option<String>,
    pub netns: Option<PodCreateNamespace>,
    pub network_options: Option<HashMap<String, String>>,
    pub networks: Option<HashMap<String, PodCreateNetworkOptions>>,
    pub no_infra: Option<bool>,
    pub no_manage_hostname: Option<bool>,
    pub no_manage_hosts: Option<bool>,
    pub no_manage_resolv_conf: Option<bool>,
    pub overlay_volumes: Option<Vec<PodCreateOverlayVolumeOptions>>,
    pub pidns: Option<PodCreateNamespace>,
    pub pod_create_command: Option<Vec<String>>,
    pub pod_devices: Option<Vec<String>>,
    pub portmappings: Option<Vec<PodCreatePortMappingOptions>>,
    pub resource_limits: Option<PodCreateResourceLimitsOptions>,
    pub restart_policy: Option<String>,
    pub restart_tries: Option<u64>,
    pub security_opt: Option<Vec<String>>,
    #[serde(rename = "serviceContainerID")]
    pub service_container_id: Option<String>,
    pub share_parent: Option<bool>,
    pub shared_namespaces: Option<Vec<String>>,
    pub shm_size: Option<i64>,
    pub shm_size_systemd: Option<i64>,
    pub sysctl: Option<HashMap<String, String>>,
    pub userns: Option<PodCreateNamespace>,
    pub utsns: Option<PodCreateNamespace>,
    pub volumes: Option<Vec<PodCreateVolumeOptions>>,
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreateIdmappingsOptions {
    #[serde(rename = "AutoUserNs")]
    pub auto_user_ns: Option<bool>,

    #[serde(rename = "AutoUserNsOpts")]
    pub auto_user_ns_opts: Option<PodCreateIdmappingsAutoUserNsOptsOptions>,

    #[serde(rename = "GIDMap")]
    pub gid_map: Option<Vec<IdMap>>,

    #[serde(rename = "HostGIDMapping")]
    pub host_gid_mapping: Option<bool>,

    #[serde(rename = "HostUIDMapping")]
    pub host_uid_mapping: Option<bool>,

    #[serde(rename = "UIDMap")]
    pub uid_map: Option<Vec<IdMap>>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreateIdmappingsAutoUserNsOptsOptions {
    #[serde(rename = "AdditionalGIDMappings")]
    pub additional_gid_mappings: Option<Vec<IdMap>>,

    #[serde(rename = "AdditionalUIDMappings")]
    pub additional_uid_mappings: Option<Vec<IdMap>>,

    #[serde(rename = "GroupFile")]
    pub group_file: Option<String>,

    #[serde(rename = "InitialSize")]
    pub initial_size: Option<u32>,

    #[serde(rename = "PasswdFile")]
    pub passwd_file: Option<String>,

    #[serde(rename = "Size")]
    pub size: Option<u32>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreateImageVolumeItemOptions {
    #[serde(rename = "Destination")]
    pub destination: Option<String>,

    #[serde(rename = "ReadWrite")]
    pub read_write: Option<bool>,

    #[serde(rename = "Source")]
    pub source: Option<String>,

    #[serde(rename = "subPath")]
    pub sub_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PodCreateNamespace {
    pub nsmode: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateMountsOptions {
    pub bind_options: Option<PodCreateMountsBindOptions>,
    pub cluster_options: Option<HashMap<String, String>>,
    pub consistency: Option<String>,
    pub image_options: Option<PodCreateMountsImageOptions>,
    pub read_only: Option<bool>,
    pub source: Option<String>,
    pub target: Option<String>,
    pub tmp_fs_options: Option<PodCreateMountsTmpFsOptions>,
    pub r#type: Option<String>,
    pub volume_options: Option<PodCreateMountsVolumeOptions>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateMountsBindOptions {
    pub create_mountpoint: Option<bool>,
    pub non_recursive: Option<bool>,
    pub propagation: Option<String>,
    pub read_only_force_recursive: Option<bool>,
    pub read_only_non_recursive: Option<bool>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateMountsImageOptions {
    pub subpath: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateMountsTmpFsOptions {
    pub mode: Option<u32>,
    pub options: Option<Vec<String>>,
    pub size_bytes: Option<i64>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateMountsVolumeOptions {
    pub driver_config: Option<PodCreateVolumeDriverConfigOptions>,
    pub labels: Option<HashMap<String, String>>,
    pub no_copy: Option<bool>,
    pub subpath: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateVolumeDriverConfigOptions {
    pub name: String,
    pub options: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreateNetworkOptions {
    pub aliases: Option<Vec<String>>,
    pub interface_name: Option<String>,
    pub options: Option<HashMap<String, String>>,
    pub static_ips: Option<Vec<String>>,
    pub static_map: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreateOverlayVolumeOptions {
    pub destination: String,
    pub options: Option<Vec<String>>,
    pub source: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreatePortMappingOptions {
    pub container_port: Option<u64>,
    pub host_ip: Option<String>,
    pub host_port: Option<u16>,
    pub protocol: Option<String>,
    pub range: Option<u16>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCreateResourceLimitsOptions {
    #[serde(rename = "blockIO")]
    pub block_io: Option<PodCreateResourceLimitsBlockIoOptions>,
    pub cpu: Option<PodCreateResourceLimitsCpuOptions>,
    pub devices: Option<Vec<PodCreateResourceLimitsDeviceOptions>>,
    pub hugepage_limits: Option<Vec<PodCreateResourceLimitsHugepageLimitOptions>>,
    pub memory: Option<PodCreateResourceLimitsMemoryOptions>,
    pub network: Option<PodCreateResourceLimitsNetworkOptions>,
    pub pids: Option<PodCreateResourceLimitsPidsOptions>,
    pub rdma: Option<HashMap<String, PodCreateResourceLimitsRdmaOptions>>,
    pub unified: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreateResourceLimitsBlockIoOptions {
    #[serde(rename = "leafWeight")]
    pub leaf_weight: Option<u16>,

    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device:
        Option<Vec<PodCreateResourceLimitsBlockIoLinuxThrottleDeviceOptions>>,

    #[serde(rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device:
        Option<Vec<PodCreateResourceLimitsBlockIoLinuxThrottleDeviceOptions>>,

    #[serde(rename = "throttleWriteBpsDevice")]
    pub throttle_write_bps_device:
        Option<Vec<PodCreateResourceLimitsBlockIoLinuxThrottleDeviceOptions>>,

    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device:
        Option<Vec<PodCreateResourceLimitsBlockIoLinuxThrottleDeviceOptions>>,

    pub weight: Option<u16>,

    pub weight_device: Option<Vec<PodCreateResourceLimitsBlockIoWeightDeviceOptions>>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreateResourceLimitsBlockIoLinuxThrottleDeviceOptions {
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub rate: Option<u64>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCreateResourceLimitsBlockIoWeightDeviceOptions {
    pub leaf_weight: Option<u16>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub weight: Option<u16>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCreateResourceLimitsCpuOptions {
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
pub struct PodCreateResourceLimitsDeviceOptions {
    pub access: Option<String>,
    pub allow: Option<bool>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub r#type: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCreateResourceLimitsHugepageLimitOptions {
    pub limit: Option<u64>,
    pub page_size: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCreateResourceLimitsMemoryOptions {
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
pub struct PodCreateResourceLimitsNetworkOptions {
    #[serde(rename = "classID")]
    pub class_id: Option<u32>,
    pub priorities: Option<Vec<PodCreateResourceLimitsNetworkPriorityOptions>>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PodCreateResourceLimitsNetworkPriorityOptions {
    pub name: Option<String>,
    pub priority: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct PodCreateResourceLimitsPidsOptions {
    limit: i64,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCreateResourceLimitsRdmaOptions {
    pub hca_handles: Option<u32>,
    pub hca_objects: Option<u32>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateVolumeOptions {
    pub dest: Option<String>,
    pub is_anonymous: Option<bool>,
    pub name: Option<String>,
    pub options: Option<String>,
    pub sub_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreate {
    pub id: String,
}

impl fmt::Debug for PodCreate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
