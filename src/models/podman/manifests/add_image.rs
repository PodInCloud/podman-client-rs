use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct ManifestAddImageOptions<'a> {
    pub name: &'a str,
    pub request: ManifestAddImageRequest,
}

#[derive(Serialize)]
pub struct ManifestAddImageRequest {
    pub all: bool,
    pub annotation: Vec<String>,
    pub annotations: HashMap<String, String>,
    pub arch: String,
    pub features: Vec<String>,
    pub images: Vec<String>,
    pub index_annotation: Vec<String>,
    pub index_annotations: HashMap<String, String>,
    pub os: String,
    pub os_features: Vec<String>,
    pub os_version: String,
    pub subject: String,
    pub variant: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ManifestAddImage {
    pub id: String,
}

impl fmt::Debug for ManifestAddImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
