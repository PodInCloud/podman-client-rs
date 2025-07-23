use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ImageCopyOptions<'a> {
    pub name: &'a str,
    pub destination: Option<&'a str>,
    pub quite: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageCopy {
    pub id: String,
}

impl fmt::Debug for ImageCopy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
