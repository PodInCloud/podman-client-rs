use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ImageVolume {
    pub destination: Option<String>,
    pub read_write: Option<bool>,
    pub source: Option<String>,
    #[serde(rename = "subPath")]
    pub sub_path: Option<String>,
}

impl fmt::Debug for ImageVolume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
