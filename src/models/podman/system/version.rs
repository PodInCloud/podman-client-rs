use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SystemVersion {
    #[serde(rename = "Platform")]
    pub platform: SystemVersionPlatform,

    #[serde(rename = "Components")]
    pub components: Vec<SystemVersionComponent>,

    #[serde(rename = "SystemVersion")]
    pub version: String,

    #[serde(rename = "SystemApiVersion")]
    pub api_version: String,

    #[serde(rename = "SystemMinAPIVersion")]
    pub min_api_version: String,

    #[serde(rename = "GitCommit")]
    pub git_commit: String,

    #[serde(rename = "SystemGoVersion")]
    pub go_version: String,

    #[serde(rename = "Os")]
    pub os: String,

    #[serde(rename = "Arch")]
    pub arch: String,

    #[serde(rename = "SystemKernelVersion")]
    pub kernel_version: String,

    #[serde(rename = "BuildTime")]
    pub build_time: String,

    #[serde(rename = "Experimental")]
    pub experimental: Option<bool>,
}

impl fmt::Debug for SystemVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemVersionPlatform {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemVersionComponent {
    pub name: String,
    pub version: String,
    pub details: HashMap<String, String>,
}
