use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct NamedVolume {
    pub dest: Option<String>,
    pub is_anonymous: Option<bool>,
    pub name: Option<String>,
    pub options: Option<String>,
    pub sub_path: Option<String>,
}

impl fmt::Debug for NamedVolume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
