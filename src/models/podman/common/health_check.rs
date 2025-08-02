use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthCheck {
    pub failing_streak: i64,
    pub log: Vec<HealthCheckLog>,
    pub status: String,
}

impl fmt::Debug for HealthCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthCheckLog {
    pub end: String,
    pub exit_code: i64,
    pub output: String,
    pub start: String,
}
