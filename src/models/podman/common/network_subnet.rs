use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NetworkSubnet {
    pub gateway: String,
    pub lease_range: NetworkSubnetLeaseRange,
    pub subnet: String,
}

impl fmt::Debug for NetworkSubnet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
pub struct NetworkSubnetLeaseRange {
    pub end_ip: String,
    pub start_ip: String,
}
