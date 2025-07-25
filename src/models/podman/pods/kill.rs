use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct PodKillOptions<'a> {
    pub name: &'a str,
    pub signal: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKill {
    pub errs: Vec<String>,
    pub id: String,
}

impl fmt::Debug for PodKill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
