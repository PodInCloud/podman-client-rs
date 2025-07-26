#[derive(Default)]
pub struct PodKubeGenerateOptions<'a> {
    pub names: Vec<&'a str>,
    pub no_trunc: Option<bool>,
    pub podman_only: Option<bool>,
    pub replicas: Option<i32>,
    pub service: Option<bool>,
    pub r#type: Option<&'a str>,
}

pub type PodKubeGenerate = String;
