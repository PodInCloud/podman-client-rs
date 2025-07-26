use crate::models::podman::images::remove::ImageRemove;

#[derive(Default)]
pub struct ImageRemoveManyOptions<'a> {
    pub all: Option<bool>,
    pub force: Option<bool>,
    pub ignore: Option<bool>,
    pub images: Option<Vec<&'a str>>,
    pub lookup_manifest: Option<bool>,
}

pub type ImageRemoveMany = ImageRemove;
