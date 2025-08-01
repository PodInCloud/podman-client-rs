use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InspectMount {
    pub destination: String,
    pub driver: String,
    pub mode: String,
    pub name: String,
    pub options: Vec<String>,
    pub propagation: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    pub source: String,
    pub sub_path: String,
    pub r#type: String,
}

impl fmt::Debug for InspectMount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
