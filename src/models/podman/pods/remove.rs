use crate::models::podman::common::pod_rm_report::PodRmReport;

#[derive(Default)]
pub struct PodRemoveOptions<'a> {
    pub name: &'a str,
    pub force: Option<bool>,
}

pub type PodRemove = PodRmReport;
