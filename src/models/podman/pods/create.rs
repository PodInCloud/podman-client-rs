use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::podman::common::{
    id_mapping_options::IdMappingOptions, image_volume::ImageVolume,
    linux_resources::LinuxResources, named_volume::NamedVolume, namespace::Namespace,
    overlay_volume::OverlayVolume, per_network_options::PerNetworkOptions,
    port_mapping::PortMapping, volume_mount::VolumeMount,
};

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
    pub idmappings: Option<IdMappingOptions>,
    pub image_volumes: Option<Vec<ImageVolume>>,
    pub infra_command: Option<Vec<String>>,
    pub infra_conmon_pid_file: Option<String>,
    pub infra_image: Option<String>,
    pub infra_name: Option<String>,
    pub ipcns: Option<Namespace>,
    pub labels: Option<HashMap<String, String>>,
    pub mounts: Option<Vec<VolumeMount>>,
    pub name: Option<String>,
    pub netns: Option<Namespace>,
    pub network_options: Option<HashMap<String, String>>,
    pub networks: Option<HashMap<String, PerNetworkOptions>>,
    pub no_infra: Option<bool>,
    pub no_manage_hostname: Option<bool>,
    pub no_manage_hosts: Option<bool>,
    pub no_manage_resolv_conf: Option<bool>,
    pub overlay_volumes: Option<Vec<OverlayVolume>>,
    pub pidns: Option<Namespace>,
    pub pod_create_command: Option<Vec<String>>,
    pub pod_devices: Option<Vec<String>>,
    pub portmappings: Option<Vec<PortMapping>>,
    pub resource_limits: Option<LinuxResources>,
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
    pub userns: Option<Namespace>,
    pub utsns: Option<Namespace>,
    pub volumes: Option<Vec<NamedVolume>>,
    pub volumes_from: Option<Vec<String>>,
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
