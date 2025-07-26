use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ImagePullOptions<'a> {
    pub all_tags: Option<bool>,
    pub arch: Option<&'a str>,
    pub compat_mode: Option<bool>,
    pub os: Option<&'a str>,
    pub policy: Option<&'a str>,
    pub quite: Option<bool>,
    pub reference: Option<&'a str>,
    pub tls_verify: Option<bool>,
    pub variant: Option<&'a str>,
    pub x_registry_auth: Option<&'a str>,
}

#[derive(Deserialize, Serialize)]
pub struct ImagePull {
    pub error: String,
    pub id: String,
    pub images: Vec<String>,
    pub stream: String,
}

impl fmt::Debug for ImagePull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
