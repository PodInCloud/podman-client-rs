use core::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct ImageInspectOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInspect {
    pub annotations: HashMap<String, String>,
    pub architecture: String,
    pub author: String,
    pub comment: String,
    pub config: ImageInspectConfig,
    pub created: DateTime<Utc>,
    pub digest: String,
    pub graph_driver: ImageInspectGraphDriver,
    pub healthcheck: ImageInspectHealthcheck,
    pub history: Vec<ImageInspectHistory>,
    pub id: String,
    pub labels: HashMap<String, String>,
    pub manifest_type: String,
    pub names_history: Vec<String>,
    pub os: String,
    pub parent: String,
    pub repo_digests: Vec<String>,
    pub repo_tags: Vec<String>,
    pub root_fs: ImageInspectRootFs,
    pub size: i64,
    pub user: String,
    pub version: String,
    pub virtual_size: i64,
}

impl fmt::Debug for ImageInspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInspectConfig {
    pub args_escaped: bool,
    pub cmd: Vec<String>,
    pub entrypoint: Vec<String>,
    pub env: Vec<String>,
    pub exposed_ports: HashMap<String, serde_json::Value>,
    pub labels: HashMap<String, String>,
    pub stop_signal: String,
    pub user: String,
    pub volumes: HashMap<String, serde_json::Value>,
    pub working_dir: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInspectGraphDriver {
    pub data: HashMap<String, String>,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInspectHealthcheck {
    pub interval: i64,
    pub retries: i64,
    pub start_interval: i64,
    pub start_period: i64,
    pub test: Vec<String>,
    pub timeout: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageInspectHistory {
    pub author: String,
    pub comment: String,
    pub created: DateTime<Utc>,
    pub created_by: String,
    pub empty_layer: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInspectRootFs {
    pub layers: Vec<String>,
    pub r#type: String,
}
