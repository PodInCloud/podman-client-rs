use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Namespace {
    pub nsmode: String,
    pub value: String,
}

impl fmt::Debug for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
