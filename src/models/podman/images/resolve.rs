use core::fmt;

use serde::{Deserialize, Serialize};

pub struct ImageResolveOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageResolve {
    pub names: Vec<String>,
}

impl fmt::Debug for ImageResolve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
