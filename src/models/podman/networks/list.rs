use crate::models::podman::networks::inspect::NetworkInspect;

#[derive(Default)]
pub struct NetworkListOptions<'a> {
    pub filters: Option<NetworkListFiltersOptions<'a>>,
}

#[derive(Default)]
pub struct NetworkListFiltersOptions<'a> {
    pub name: Option<Vec<&'a str>>,
    pub id: Option<Vec<&'a str>>,
    pub driver: Option<Vec<&'a str>>,
    pub label: Option<Vec<&'a str>>,
    pub until: Option<Vec<&'a str>>,
}

pub type NetworkList = Vec<NetworkInspect>;
