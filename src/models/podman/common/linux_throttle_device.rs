use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct LinuxThrottleDevice {
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub rate: Option<u64>,
}

impl fmt::Debug for LinuxThrottleDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
