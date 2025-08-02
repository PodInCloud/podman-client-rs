use crate::models::podman::artifacts::pull::ArtifactPull;

pub struct ArtifactRemoveOptions<'a> {
    pub name: &'a str,
}

pub type ArtifactRemove = ArtifactPull;
