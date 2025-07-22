use crate::models::podman::secrets::inspect::SecretInspect;

#[derive(Default)]
pub struct SecretListOptions<'a> {
    pub filters: Option<SecretListFiltersOptions<'a>>,
}

#[derive(Default)]
pub struct SecretListFiltersOptions<'a> {
    pub name: Option<Vec<&'a str>>,
    pub id: Option<Vec<&'a str>>,
}

pub type SecretList = Vec<SecretInspect>;
