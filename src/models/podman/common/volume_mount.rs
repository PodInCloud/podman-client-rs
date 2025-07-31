use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeMount {
    pub bind_options: Option<VolumeMountBindOptions>,
    pub cluster_options: Option<HashMap<String, String>>,
    pub consistency: Option<String>,
    pub image_options: Option<VolumeMountImageOptions>,
    pub read_only: Option<bool>,
    pub source: Option<String>,
    pub target: Option<String>,
    pub tmp_fs_options: Option<VolumeMountTmpFsOptions>,
    pub r#type: Option<String>,
    pub volume_options: Option<VolumeMountVolumeOptions>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeMountBindOptions {
    pub create_mountpoint: Option<bool>,
    pub non_recursive: Option<bool>,
    pub propagation: Option<String>,
    pub read_only_force_recursive: Option<bool>,
    pub read_only_non_recursive: Option<bool>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeMountImageOptions {
    pub subpath: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeMountTmpFsOptions {
    pub mode: Option<u32>,
    pub options: Option<Vec<String>>,
    pub size_bytes: Option<i64>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeMountVolumeOptions {
    pub driver_config: Option<VolumeMountOptionsDriverConfig>,
    pub labels: Option<HashMap<String, String>>,
    pub no_copy: Option<bool>,
    pub subpath: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeMountOptionsDriverConfig {
    pub name: String,
    pub options: Option<HashMap<String, String>>,
}
