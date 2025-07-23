use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct VolumePruneOptions<'a> {
    pub filters: Option<VolumePruneFiltersOptions<'a>>,
}

#[derive(Default)]
pub struct VolumePruneFiltersOptions<'a> {
    pub until: Option<&'a str>,
    pub label: Option<&'a str>,
    pub labelnot: Option<&'a str>,
}

pub type VolumePrune = Vec<VolumePruneItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumePruneItem {
    pub err: Option<String>,
    pub id: String,
    pub size: u64,
}

impl fmt::Debug for VolumePruneItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
