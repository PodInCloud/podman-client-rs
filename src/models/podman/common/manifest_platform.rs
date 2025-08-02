use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ManifestPlatform {
    pub architecture: String,
    pub features: Vec<String>,
    pub os: String,
    #[serde(rename = "os.features")]
    pub os_features: Vec<String>,
    #[serde(rename = "os.version")]
    pub os_version: String,
    pub variant: String,
}

impl fmt::Debug for ManifestPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
