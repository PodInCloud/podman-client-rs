use crate::models::podman::pods::kube_play::PodKubePlay;

#[derive(Default)]
pub struct PodKubePlayRemoveOptions {
    pub force: Option<bool>,
}

pub type PodKubePlayRemove = PodKubePlay;
