use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::podman::{
    common::{network_route::NetworkRoute, network_subnet::NetworkSubnet},
    networks::inspect::NetworkInspect,
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
    pub routes: Vec<NetworkRoute>,
    pub subnets: Vec<NetworkSubnet>,
}

pub type NetworkCreate = NetworkInspect;
