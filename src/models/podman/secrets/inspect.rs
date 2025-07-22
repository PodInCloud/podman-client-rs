use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct SecretInspectOptions {
    pub name: String,
    pub show_secret: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct SecretInspect {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "CreatedAt")]
    pub created_at: String,

    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,

    #[serde(rename = "Spec")]
    pub spec: SecretInspectSpec,

    #[serde(rename = "SecretData")]
    pub secret_data: Option<String>,
}

impl fmt::Debug for SecretInspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretInspectSpec {
    pub name: String,
    pub driver: SecretInspectSpecDriver,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretInspectSpecDriver {
    pub name: String,
    pub options: HashMap<String, String>,
}
