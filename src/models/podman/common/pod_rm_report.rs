use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodRmReport {
    pub err: String,
    pub id: String,
    pub removed_ctrs: HashMap<String, String>,
}

impl fmt::Debug for PodRmReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
