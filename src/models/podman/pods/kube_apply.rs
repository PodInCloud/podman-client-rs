#[derive(Default)]
pub struct PodKubeApplyOptions<'a> {
    pub ca_cert_file: Option<&'a str>,
    pub file: Option<&'a str>,
    pub kube_config: Option<&'a str>,
    pub namespace: Option<&'a str>,
    pub service: Option<bool>,
    pub kubernetes_yaml_file: String,
}

pub type PodKubeApply = String;
