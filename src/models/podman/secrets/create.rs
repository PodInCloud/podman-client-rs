use core::fmt;

use serde::{Deserialize, Serialize};

pub struct SecretCreateOptions<'a> {
    pub driver: Option<&'a str>,
    pub driver_opts: Option<&'a str>,
    pub labels: Option<&'a str>,
    pub name: &'a str,
    pub secret: &'a str,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SecretCreate {
    pub id: String,
}

impl fmt::Debug for SecretCreate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
