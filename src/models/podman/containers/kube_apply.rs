use crate::models::podman::pods::kube_apply::{PodKubeApply, PodKubeApplyOptions};

pub type ContainerKubeApplyOptions<'a> = PodKubeApplyOptions<'a>;

pub type ContainerKubeApply = PodKubeApply;
