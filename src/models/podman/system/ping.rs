use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemPing {
    pub ostype: Option<String>,
    pub buildkit_version: String,
    pub api_version: String,
    pub cache_control: String,
    pub docker_experimental: String,
    pub pragma: String,
    pub builder_version: Option<String>,
    pub server: Option<String>,
    pub libpod_api_version: String,
    pub x_reference_id: Option<String>,
    pub date: Option<String>,
    pub libpod_buildah_version: String,
}

impl fmt::Debug for SystemPing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
