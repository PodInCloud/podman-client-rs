use core::fmt;

use serde::{Deserialize, Serialize};

use crate::models::podman::manifests::create::ManifestCreateRequest;

pub struct ManifestModifyOptions<'a> {
    pub name: &'a str,
    pub tls_verify: Option<bool>,
    pub request: ManifestModifyRequest,
}

pub type ManifestModifyRequest = ManifestCreateRequest;

#[derive(Deserialize, Serialize)]
pub struct ManifestModify {
    pub errors: Vec<String>,
    pub files: Vec<String>,
    #[serde(rename = "Id")]
    pub id: String,
    pub images: Vec<String>,
}

impl fmt::Debug for ManifestModify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
