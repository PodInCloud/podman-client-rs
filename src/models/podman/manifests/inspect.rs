use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ManifestInspectOptions<'a> {
    pub name: &'a str,
    pub tls_verify: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestInspect {
    pub manifests: Vec<ManifestInspectManifest>,
    pub media_type: String,
    pub schema_version: i64,
}

impl fmt::Debug for ManifestInspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestInspectManifest {
    pub digest: String,
    pub media_type: String,
    pub platform: ManifestInspectManifestPlatform,
    pub size: i64,
    pub urls: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ManifestInspectManifestPlatform {
    pub architecture: String,
    pub features: Vec<String>,
    pub os: String,
    #[serde(rename = "os.features")]
    pub os_features: Vec<String>,
    #[serde(rename = "os.version")]
    pub os_version: String,
    pub variant: String,
}
