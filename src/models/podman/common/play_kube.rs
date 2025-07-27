use core::fmt;

use serde::{Deserialize, Serialize};

use crate::models::podman::common::pod_rm_report::PodRmReport;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayKube {
    pub exit_code: i32,
    pub pods: Vec<PlayKubePod>,
    pub rm_report: Vec<PodRmReport>,
    pub secret_rm_report: Vec<PlayKubeSecretRmReport>,
    pub secrets: Vec<PlayKubeSecret>,
    #[serde(rename = "ServiceContainerID")]
    pub service_container_id: String,
    pub stop_report: Vec<PlayKubeStopReport>,
    pub volume_rm_report: Vec<PlayKubeVolumeRmReport>,
    pub volumes: Vec<PlayKubeVolume>,
}

impl fmt::Debug for PlayKube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayKubePod {
    pub container_errors: Vec<String>,
    pub containers: Vec<String>,
    #[serde(rename = "ID")]
    pub id: String,
    pub init_containers: Vec<String>,
    pub logs: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayKubeSecretRmReport {
    pub err: String,
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayKubeSecret {
    pub create_report: PlayKubeSecretCreateReport,
}

#[derive(Deserialize, Serialize)]
pub struct PlayKubeSecretCreateReport {
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayKubeStopReport {
    pub errs: Vec<String>,
    pub id: String,
    pub raw_input: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayKubeVolumeRmReport {
    pub err: String,
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayKubeVolume {
    pub name: String,
}
