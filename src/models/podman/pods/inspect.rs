use core::fmt;
use std::{collections::HashMap, net::Ipv4Addr};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct PodInspectOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct PodInspect {
    pub blkio_weight: u64,
    pub blkio_weight_device: Vec<PodInspectBlkioWeightDevice>,
    #[serde(rename = "CgroupParent")]
    pub cgroup_parent: String,
    #[serde(rename = "CgroupPath")]
    pub cgroup_path: String,
    #[serde(rename = "Containers")]
    pub containers: Vec<PodInspectContainer>,
    pub cpu_period: u64,
    pub cpu_quota: i64,
    pub cpu_shares: u64,
    pub cpuset_cpus: String,
    pub cpuset_mems: String,
    #[serde(rename = "CreateCgroup")]
    pub create_cgroup: bool,
    #[serde(rename = "CreateCommand")]
    pub create_command: Vec<String>,
    #[serde(rename = "Created")]
    pub created: DateTime<Utc>,
    #[serde(rename = "CreateInfra")]
    pub create_infra: bool,
    pub device_read_bps: Vec<PodCreateBlkioThrottleDevice>,
    pub device_write_bps: Vec<PodCreateBlkioThrottleDevice>,
    #[serde(rename = "Devices")]
    pub devices: Vec<PodCreateDevice>,
    #[serde(rename = "ExitPolicy")]
    pub exit_policy: String,
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "InfraConfig")]
    pub infra_config: PodCreateInfraConfig,
    #[serde(rename = "InfraContainerID")]
    pub infra_container_id: String,
    pub labels: HashMap<String, String>,
    #[serde(rename = "LockNumber")]
    pub lock_number: u32,
    pub memory_limit: u64,
    pub memory_swap: u64,
    pub mounts: Vec<PodCreateMount>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "NumContainers")]
    pub num_containers: u64,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: String,
    pub security_opt: Vec<String>,
    #[serde(rename = "SharedNamespaces")]
    pub shared_namespaces: Vec<String>,
    #[serde(rename = "State")]
    pub state: String,
    pub volumes_from: Vec<String>,
}

impl fmt::Debug for PodInspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PodInspectBlkioWeightDevice {
    pub path: String,
    pub weight: u16,
}

#[derive(Deserialize, Serialize)]
pub struct PodInspectContainer {
    pub id: String,
    pub name: String,
    pub state: String,
}

#[derive(Deserialize, Serialize)]
pub struct PodCreateBlkioThrottleDevice {
    pub path: String,
    pub rate: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateDevice {
    pub cgroup_permissions: String,
    pub path_in_container: String,
    pub path_in_host: String,
}

#[derive(Deserialize, Serialize)]
pub struct PodCreateInfraConfig {
    pub cpu_period: u64,
    pub cpu_quota: i64,
    pub cpuset_cpus: String,
    #[serde(rename = "DNSOption")]
    pub dns_option: Vec<String>,
    #[serde(rename = "DNSSearch")]
    pub dns_search: Vec<String>,
    #[serde(rename = "DNSServer")]
    pub dns_server: Vec<String>,
    #[serde(rename = "HostNetwork")]
    pub host_network: bool,
    #[serde(rename = "HostsFile")]
    pub hosts_file: String,
    #[serde(rename = "NetworkOptions")]
    pub network_options: HashMap<String, Vec<String>>,
    #[serde(rename = "Networks")]
    pub networks: Vec<String>,
    #[serde(rename = "NoManageHostname")]
    pub no_manage_hostname: bool,
    #[serde(rename = "NoManageHosts")]
    pub no_manage_hosts: bool,
    #[serde(rename = "NoManageResolvConf")]
    pub no_manage_resolv_conf: bool,
    pub pid_ns: String,
    #[serde(rename = "PortBindings")]
    pub port_bindings: HashMap<String, Vec<PodCreateInfraConfigPortBinding>>,
    #[serde(rename = "StaticIP")]
    pub static_ip: Ipv4Addr,
    #[serde(rename = "StaticMAC")]
    pub static_mac: String,
    pub userns: String,
    pub uts_ns: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateInfraConfigPortBinding {
    pub host_ip: String,
    pub host_port: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodCreateMount {
    pub destination: String,
    pub driver: String,
    pub mode: String,
    pub name: String,
    pub options: Vec<String>,
    pub propagation: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    pub source: String,
    pub sub_path: String,
    pub r#type: String,
}
