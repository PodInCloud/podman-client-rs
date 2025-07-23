use crate::models::podman::volumes::inspect::VolumeInspect;

#[derive(Default)]
pub struct VolumeListOptions<'a> {
    pub filters: Option<VolumeListFiltersOptions<'a>>,
}

#[derive(Default)]
pub struct VolumeListFiltersOptions<'a> {
    pub driver: Option<Vec<&'a str>>,
    pub label: Option<Vec<&'a str>>,
    pub name: Option<Vec<&'a str>>,
    pub opt: Option<Vec<&'a str>>,
    pub until: Option<Vec<&'a str>>,
}

pub type VolumeList = Vec<VolumeInspect>;
