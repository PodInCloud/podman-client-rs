use std::collections::HashMap;

use serde::Serialize;

use crate::models::podman::volumes::inspect::VolumeInspect;

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

pub type VolumeCreate = VolumeInspect;
