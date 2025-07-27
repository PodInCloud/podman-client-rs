use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::podman::networks::inspect::{
    NetworkInspect, NetworkInspectRoute, NetworkInspectSubnet,
};

#[derive(Serialize)]
pub struct NetworkCreateOptions {
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

pub type NetworkCreate = NetworkInspect;
