use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Version {
    #[serde(rename = "Platform")]
    pub platform: VersionPlatform,

    #[serde(rename = "Components")]
    pub components: Vec<VersionComponent>,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "ApiVersion")]
    pub api_version: String,

    #[serde(rename = "MinAPIVersion")]
    pub min_api_version: String,

    #[serde(rename = "GitCommit")]
    pub git_commit: String,

    #[serde(rename = "GoVersion")]
    pub go_version: String,

    #[serde(rename = "Os")]
    pub os: String,

    #[serde(rename = "Arch")]
    pub arch: String,

    #[serde(rename = "KernelVersion")]
    pub kernel_version: String,

    #[serde(rename = "BuildTime")]
    pub build_time: String,
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VersionPlatform {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VersionComponent {
    pub name: String,
    pub version: String,
    pub details: VersionComponentDetails,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum VersionComponentDetails {
    PodmanEngine(VersionComponentDetailsPodmanEngine),
    Else(VersionComponentDetailsElse),
}

#[derive(Deserialize, Serialize)]
pub struct VersionComponentDetailsPodmanEngine {
    #[serde(rename = "APIVersion")]
    pub api_version: String,

    #[serde(rename = "Arch")]
    pub arch: String,

    #[serde(rename = "BuildTime")]
    pub build_time: String,

    #[serde(rename = "Experimental")]
    pub experimental: String,

    #[serde(rename = "GitCommit")]
    pub git_commit: String,

    #[serde(rename = "GoVersion")]
    pub go_version: String,

    #[serde(rename = "KernelVersion")]
    pub kernel_version: String,

    #[serde(rename = "MinAPIVersion")]
    pub min_api_version: String,

    #[serde(rename = "Os")]
    pub os: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VersionComponentDetailsElse {
    pub package: String,
}
