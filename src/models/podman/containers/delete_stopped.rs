use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ContainerDeleteStoppedOptions<'a> {
    pub filters: Option<ContainerDeleteStoppedFiltersOptions<'a>>,
}

#[derive(Default)]
pub struct ContainerDeleteStoppedFiltersOptions<'a> {
    pub until: Option<Vec<&'a str>>,
    pub label: Option<Vec<&'a str>>,
    pub labelnot: Option<Vec<&'a str>>,
}

pub type ContainerDeleteStopped = Vec<ContainerDeleteStoppedItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerDeleteStoppedItem {
    pub err: Option<String>,
    pub id: String,
    pub size: i64,
}

impl fmt::Debug for ContainerDeleteStoppedItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
