use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct OverlayVolume {
    pub destination: String,
    pub options: Option<Vec<String>>,
    pub source: String,
}

impl fmt::Debug for OverlayVolume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
