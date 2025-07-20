use core::fmt;

use serde::{Deserialize, Serialize};

pub struct CreateSecretOptions {
    pub driver: Option<String>,
    pub driveropts: Option<String>,
    pub labels: Option<String>,
    pub name: String,
    pub secret: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct CreateSecret {
    pub id: String,
}

impl fmt::Debug for CreateSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
