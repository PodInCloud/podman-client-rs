use crate::models::podman::pods::kube_generate::{PodKubeGenerate, PodKubeGenerateOptions};

pub type ContainerKubeGenerateOptions<'a> = PodKubeGenerateOptions<'a>;

pub type ContainerKubeGenerate = PodKubeGenerate;
