use core::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct PodListOptions<'a> {
    pub filters: Option<PodListFiltersOptions<'a>>,
}

#[derive(Default)]
pub struct PodListFiltersOptions<'a> {
    pub id: Option<Vec<&'a str>>,
    pub label: Option<Vec<&'a str>>,
    pub name: Option<Vec<&'a str>>,
    pub until: Option<Vec<&'a str>>,
    pub status: Option<Vec<PodListFiltersStatusOptions>>,
    pub network: Option<Vec<&'a str>>,
    pub ctr_names: Option<Vec<&'a str>>,
    pub ctr_ids: Option<Vec<&'a str>>,
    pub ctr_status: Option<Vec<&'a str>>,
    pub ctr_number: Option<Vec<&'a str>>,
}

pub enum PodListFiltersStatusOptions {
    Stopped,
    Running,
    Paused,
    Exited,
    Dead,
    Created,
    Degraded,
}

impl PodListFiltersStatusOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stopped => "stopped",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Exited => "exited",
            Self::Dead => "dead",
            Self::Created => "created",
            Self::Degraded => "degraded",
        }
    }
}

pub type PodList = Vec<PodListItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodListItem {
    pub cgroup: String,
    pub containers: Vec<PodListItemContainer>,
    pub created: DateTime<Utc>,
    pub id: String,
    pub infra_id: String,
    pub labels: HashMap<String, String>,
    pub name: String,
    pub namespace: String,
    pub networks: Vec<String>,
    pub status: String,
}

impl fmt::Debug for PodListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodListItemContainer {
    pub id: String,
    pub names: String,
    pub restart_count: u64,
    pub status: String,
}
