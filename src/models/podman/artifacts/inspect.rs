use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::podman::common::manifest_platform::ManifestPlatform;

pub struct ArtifactInspectOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtifactInspect {
    pub digest: String,
    pub manifest: ArtifactInspectManifest,
    pub name: String,
}

impl fmt::Debug for ArtifactInspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactInspectManifest {
    pub annotations: HashMap<String, String>,
    pub artifact_type: String,
    pub config: ArtifactInspectManifestDescriptor,
    pub layers: Vec<ArtifactInspectManifestDescriptor>,
    pub media_type: String,
    pub schema_version: i64,
    pub subject: ArtifactInspectManifestDescriptor,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactInspectManifestDescriptor {
    pub annotations: HashMap<String, String>,
    pub artifact_type: String,
    pub data: Vec<u8>,
    pub digest: String,
    pub media_type: String,
    pub platform: ManifestPlatform,
    pub size: i64,
    pub urls: Vec<String>,
}
