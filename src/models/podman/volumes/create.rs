use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
pub struct VolumeCreateOptions<'a> {
    #[serde(rename = "Driver")]
    pub driver: Option<&'a str>,

    #[serde(rename = "GID")]
    pub gid: Option<i64>,

    #[serde(rename = "IgnoreIfExists")]
    pub ignore_if_exists: Option<bool>,

    #[serde(rename = "Label")]
    pub label: Option<HashMap<&'a str, &'a str>>,

    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<&'a str, &'a str>>,

    #[serde(rename = "Name")]
    pub name: Option<&'a str>,

    #[serde(rename = "Options")]
    pub options: Option<HashMap<&'a str, &'a str>>,

    #[serde(rename = "UID")]
    pub uid: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct VolumeCreate {
    #[serde(rename = "Anonymous")]
    pub anonymous: Option<bool>,

    #[serde(rename = "CreatedAt")]
    pub created_at: String,

    #[serde(rename = "Driver")]
    pub driver: String,

    #[serde(rename = "GID")]
    pub gid: Option<i64>,

    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "LockNumber")]
    pub lock_number: u32,

    #[serde(rename = "MountCount")]
    pub mount_count: u64,

    #[serde(rename = "MountPoint")]
    pub mount_point: Option<String>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "NeedsChown")]
    pub needs_chown: bool,

    #[serde(rename = "NeedsCopyUp")]
    pub needs_copy_up: bool,

    #[serde(rename = "Options")]
    pub options: HashMap<String, String>,

    #[serde(rename = "Scope")]
    pub scope: String,

    #[serde(rename = "Status")]
    pub status: Option<HashMap<String, serde_json::Value>>,

    #[serde(rename = "Timeout")]
    pub timeout: Option<u64>,

    #[serde(rename = "UID")]
    pub uid: Option<i64>,
}

impl fmt::Debug for VolumeCreate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
