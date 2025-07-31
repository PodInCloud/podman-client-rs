use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Schema2HealthConfig {
    pub interval: i64,
    pub retries: i64,
    pub start_interval: i64,
    pub start_period: i64,
    pub test: Vec<String>,
    pub timeout: i64,
}

impl fmt::Debug for Schema2HealthConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
