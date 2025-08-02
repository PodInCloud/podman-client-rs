use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ArtifactPullOptions<'a> {
    pub name: &'a str,
    pub retry: Option<i64>,
    pub retry_delay: Option<&'a str>,
    pub tls_verify: Option<bool>,
    pub x_registry_auth: Option<&'a str>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtifactPull {
    pub artifact_digest: String,
}

impl fmt::Debug for ArtifactPull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
