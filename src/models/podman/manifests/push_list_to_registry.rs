use core::fmt;

use serde::{Deserialize, Serialize};

pub struct ManifestPushListToRegistryOptions<'a> {
    pub destination: &'a str,
    pub name: &'a str,
    pub add_compression: Option<Vec<&'a str>>,
    pub all: Option<bool>,
    pub force_compression_format: Option<bool>,
    pub quiet: Option<bool>,
    pub tls_verify: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ManifestPushListToRegistry {
    pub id: String,
}

impl fmt::Debug for ManifestPushListToRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
