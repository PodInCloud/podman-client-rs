use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ContainerListProcessesOptions<'a> {
    pub name: &'a str,
    pub delay: Option<i64>,
    pub ps_args: Option<Vec<&'a str>>,
    pub stream: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerListProcesses {
    pub processes: Vec<String>,
    pub titles: Vec<String>,
}

impl fmt::Debug for ContainerListProcesses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
