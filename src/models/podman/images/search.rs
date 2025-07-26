use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ImageSearchOptions<'a> {
    pub filters: Option<ImageSearchFiltersOptions>,
    pub limit: Option<i64>,
    pub list_tags: Option<bool>,
    pub term: Option<&'a str>,
    pub tls_verify: Option<bool>,
}

#[derive(Default)]
pub struct ImageSearchFiltersOptions {
    pub is_automated: Option<Vec<bool>>,
    pub is_official: Option<Vec<bool>>,
    pub stars: Option<Vec<i64>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageSearch {
    pub automated: String,
    pub description: String,
    pub index: String,
    pub name: String,
    pub official: String,
    pub stars: i64,
    pub tag: String,
}

impl fmt::Debug for ImageSearch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
