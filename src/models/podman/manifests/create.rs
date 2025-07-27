use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct ManifestCreateOptions<'a> {
    pub name: &'a str,
    pub all: Option<bool>,
    pub amend: Option<bool>,
    pub images: Vec<&'a str>,
    pub request: ManifestCreateRequest,
}

#[derive(Serialize)]
pub struct ManifestCreateRequest {
    pub all: bool,
    pub annotation: Vec<String>,
    pub annotations: HashMap<String, String>,
    pub arch: String,
    pub artifact_annotations: HashMap<String, String>,
    pub artifact_config: String,
    pub artifact_config_type: String,
    pub artifact_exclude_titles: bool,
    pub artifact_files: Vec<String>,
    pub artifact_layer_type: String,
    pub artifact_subject: String,
    pub artifact_type: String,
    pub features: Vec<String>,
    pub images: Vec<String>,
    pub index_annotation: Vec<String>,
    pub index_annotations: HashMap<String, String>,
    pub operation: String,
    pub os: String,
    pub os_features: Vec<String>,
    pub os_version: String,
    pub subject: String,
    pub variant: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ManifestCreate {
    pub id: String,
}

impl fmt::Debug for ManifestCreate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
