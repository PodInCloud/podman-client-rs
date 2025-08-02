use crate::models::podman::artifacts::pull::{ArtifactPull, ArtifactPullOptions};

pub type ArtifactPushOptions<'a> = ArtifactPullOptions<'a>;

pub type ArtifactPush = ArtifactPull;
