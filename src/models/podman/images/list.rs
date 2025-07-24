use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ImageListOptions<'a> {
    pub all: Option<bool>,
    pub filters: Option<ImageListFiltersOptions<'a>>,
}

#[derive(Default)]
pub struct ImageListFiltersOptions<'a> {
    pub before: Option<Vec<&'a str>>,
    pub dangling: Option<Vec<bool>>,
    pub label: Option<Vec<&'a str>>,
    pub reference: Option<Vec<&'a str>>,
    pub id: Option<Vec<&'a str>>,
    pub since: Option<Vec<&'a str>>,
}

pub type ImageList = Vec<ImageListItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageListItem {
    pub arch: String,
    pub containers: i64,
    pub created: i64,
    pub dangling: bool,
    pub digest: String,
    pub history: Vec<String>,
    pub id: String,
    pub is_manifest_list: bool,
    pub labels: HashMap<String, String>,
    pub names: Vec<String>,
    pub os: String,
    pub parent_id: String,
    pub read_only: bool,
    pub repo_digests: Vec<String>,
    pub repo_tags: Vec<String>,
    pub shared_size: i64,
    pub size: i64,
    pub virtual_size: i64,
}
