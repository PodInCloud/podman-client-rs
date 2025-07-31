use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxWeightDevice {
    pub leaf_weight: Option<u16>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub weight: Option<u16>,
}

impl fmt::Debug for LinuxWeightDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
