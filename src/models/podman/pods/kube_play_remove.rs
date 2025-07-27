use crate::models::podman::common::play_kube::PlayKube;

#[derive(Default)]
pub struct PodKubePlayRemoveOptions {
    pub force: Option<bool>,
}

pub type PodKubePlayRemove = PlayKube;
