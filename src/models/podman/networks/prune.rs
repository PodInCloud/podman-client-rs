use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct NetworkPruneOptions<'a> {
    pub filters: Option<NetworkPruneFiltersOptions<'a>>,
}

#[derive(Default)]
pub struct NetworkPruneFiltersOptions<'a> {
    pub until: Option<Vec<&'a str>>,
    pub label: Option<Vec<&'a str>>,
    pub labelnot: Option<Vec<&'a str>>,
}

pub type NetworkPrune = Vec<NetworkPruneItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkPruneItem {
    pub error: String,
    pub name: String,
}

impl fmt::Debug for NetworkPruneItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
