use std::collections::HashMap;

use crate::models::podman::common::play_kube::PlayKube;

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

pub type PodKubePlay = PlayKube;
