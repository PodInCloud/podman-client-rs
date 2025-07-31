use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct PerNetworkOptions {
    pub aliases: Option<Vec<String>>,
    pub interface_name: Option<String>,
    pub options: Option<HashMap<String, String>>,
    pub static_ips: Option<Vec<String>>,
    pub static_map: Option<String>,
}

impl fmt::Debug for PerNetworkOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
