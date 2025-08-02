use core::fmt;

use serde::{Deserialize, Serialize};

use crate::models::podman::common::manifest_platform::ManifestPlatform;

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
    pub platform: ManifestPlatform,
    pub size: i64,
    pub urls: Vec<String>,
}
