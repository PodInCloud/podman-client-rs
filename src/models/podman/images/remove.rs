use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ImageRemoveOptions<'a> {
    pub name: &'a str,
    pub force: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageRemove {
    pub deleted: Vec<String>,
    pub errors: Vec<String>,
    pub exit_code: i64,
    pub untagged: Vec<String>,
}

impl fmt::Debug for ImageRemove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
