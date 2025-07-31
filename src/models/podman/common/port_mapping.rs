use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct PortMapping {
    pub container_port: Option<u64>,
    pub host_ip: Option<String>,
    pub host_port: Option<u16>,
    pub protocol: Option<String>,
    pub range: Option<u16>,
}

impl fmt::Debug for PortMapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
