use core::fmt;

use serde::{Deserialize, Serialize};

pub struct ImageHistoryOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageHistory {
    pub comment: String,
    pub created: i64,
    pub created_by: String,
    pub id: String,
    pub size: i64,
    pub tags: Vec<String>,
}

impl fmt::Debug for ImageHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
