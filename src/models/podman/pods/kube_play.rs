use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct PodKubePlayOptions<'a> {
    pub annotation: Option<HashMap<&'a str, &'a str>>,
    pub build: Option<bool>,
    pub log_driver: Option<&'a str>,
    pub log_options: Option<Vec<&'a str>>,
    pub network: Option<Vec<&'a str>>,
    pub no_hosts: Option<bool>,
    pub no_trunc: Option<bool>,
    pub publish_all_ports: Option<bool>,
    pub publish_ports: Option<Vec<&'a str>>,
    pub replace: Option<bool>,
    pub service_container: Option<bool>,
    pub start: Option<bool>,
    pub static_ips: Option<Vec<&'a str>>,
    pub static_macs: Option<Vec<&'a str>>,
    pub tls_verify: Option<bool>,
    pub userns: Option<&'a str>,
    pub wait: Option<bool>,
    pub kubernetes_yaml_file: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKubePlay {
    pub exit_code: i32,
    pub pods: Vec<PodKubePlayPod>,
    pub rm_report: Vec<PodKubePlayRmReport>,
    pub secret_rm_report: Vec<PodKubePlaySecretRmReport>,
    pub secrets: Vec<PodKubePlaySecret>,
    #[serde(rename = "ServiceContainerID")]
    pub service_container_id: String,
    pub stop_report: Vec<PodKubePlayStopReport>,
    pub volume_rm_report: Vec<PodKubePlayVolumeRmReport>,
    pub volumes: Vec<PodKubePlayVolume>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKubePlayPod {
    pub container_errors: Vec<String>,
    pub containers: Vec<String>,
    pub id: String,
    pub init_containers: Vec<String>,
    pub logs: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKubePlayRmReport {
    pub err: String,
    pub id: String,
    pub removed_ctrs: HashMap<String, String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKubePlaySecretRmReport {
    pub err: String,
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKubePlaySecret {
    pub create_report: PodKubePlaySecretCreateReport,
}

#[derive(Deserialize, Serialize)]
pub struct PodKubePlaySecretCreateReport {
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKubePlayStopReport {
    pub errs: Vec<String>,
    pub id: String,
    pub raw_input: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKubePlayVolumeRmReport {
    pub err: String,
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKubePlayVolume {
    pub name: String,
}
