use core::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct NetworkInspectOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct NetworkInspect {
    pub containers: HashMap<String, NetworkInspectContainer>,
    pub created: DateTime<Utc>,
    pub dns_enabled: bool,
    pub driver: String,
    pub id: String,
    pub internal: bool,
    pub ipam_options: HashMap<String, String>,
    pub ipv6_enabled: bool,
    pub labels: HashMap<String, String>,
    pub name: String,
    pub network_dns_servers: Vec<String>,
    pub network_interface: String,
    pub options: HashMap<String, String>,
    pub routes: Vec<NetworkInspectRoute>,
    pub subnets: Vec<NetworkInspectSubnet>,
}

impl fmt::Debug for NetworkInspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
pub struct NetworkInspectContainer {
    pub interfaces: HashMap<String, NetworkInspectContainerInterface>,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct NetworkInspectContainerInterface {
    pub mac_address: String,
    pub subnets: Vec<NetworkInspectContainerInterfaceSubnet>,
}

#[derive(Deserialize, Serialize)]
pub struct NetworkInspectContainerInterfaceSubnet {
    pub gateway: String,
    pub ipnet: NetworkInspectContainerInterfaceSubnetIpnet,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkInspectContainerInterfaceSubnetIpnet {
    #[serde(rename = "IP")]
    pub ip: String,
    pub mask: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct NetworkInspectRoute {
    pub destination: String,
    pub gateway: String,
    pub metric: u32,
}

#[derive(Deserialize, Serialize)]
pub struct NetworkInspectSubnet {
    pub gateway: String,
    pub lease_range: NetworkInspectSubnetLeaseRange,
    pub subnet: String,
}

#[derive(Deserialize, Serialize)]
pub struct NetworkInspectSubnetLeaseRange {
    pub end_ip: String,
    pub start_ip: String,
}
