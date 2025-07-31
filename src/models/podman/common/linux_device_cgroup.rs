use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct LinuxDeviceCgroup {
    pub access: Option<String>,
    pub allow: Option<bool>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub r#type: Option<String>,
}

impl fmt::Debug for LinuxDeviceCgroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
