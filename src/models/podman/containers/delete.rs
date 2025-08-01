use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ContainerDeleteOptions<'a> {
    pub name: &'a str,
    pub depend: Option<bool>,
    pub force: Option<bool>,
    pub ignore: Option<bool>,
    pub timeout: Option<i64>,
    pub v: Option<bool>,
}

pub type ContainerDelete = Vec<ContainerDeleteItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerDeleteItem {
    pub err: Option<String>,
    pub id: String,
}

impl fmt::Debug for ContainerDeleteItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
