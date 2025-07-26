use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ImagePruneOptions<'a> {
    pub all: Option<bool>,
    pub build_cache: Option<bool>,
    pub external: Option<bool>,
    pub filters: Option<ImagePruneFiltersOptions<'a>>,
}

#[derive(Default)]
pub struct ImagePruneFiltersOptions<'a> {
    pub dangling: Option<Vec<bool>>,
    pub until: Option<Vec<&'a str>>,
    pub label: Option<Vec<&'a str>>,
    pub labelnot: Option<Vec<&'a str>>,
}

pub type ImagePrune = Vec<ImagePruneItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImagePruneItem {
    pub err: String,
    pub id: String,
    pub size: u64,
}

impl fmt::Debug for ImagePruneItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
