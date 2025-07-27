use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct PodStopOptions<'a> {
    pub name: &'a str,
    pub t: Option<i64>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodStop {
    pub errs: Vec<String>,
    pub id: String,
    pub raw_input: String,
}

impl fmt::Debug for PodStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
