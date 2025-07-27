use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct NetworkRemoveOptions<'a> {
    pub name: &'a str,
    pub force: Option<bool>,
}

pub type NetworkRemove = Vec<NetworkRemoveItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkRemoveItem {
    pub err: String,
    pub name: String,
}

impl fmt::Debug for NetworkRemoveItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
