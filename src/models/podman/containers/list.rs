use core::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::podman::common::port_mapping::PortMapping;

#[derive(Default)]
pub struct ContainerListOptions {
    pub all: Option<bool>,
    pub filters: Option<ContainerListFiltersOptions>,
    pub limit: Option<i64>,
    pub namespace: Option<bool>,
    pub pod: Option<bool>,
    pub size: Option<bool>,
    pub sync: Option<bool>,
}

#[derive(Default)]
pub struct ContainerListFiltersOptions {
    pub ancestor: Option<Vec<String>>,
    pub before: Option<Vec<String>>,
    pub expose: Option<Vec<String>>,
    pub exited: Option<Vec<i64>>,
    pub health: Option<Vec<ContainerListFiltersHealthOptions>>,
    pub id: Option<Vec<String>>,
    pub is_task: Option<Vec<bool>>,
    pub label: Option<Vec<String>>,
    pub name: Option<Vec<String>>,
    pub network: Option<Vec<String>>,
    pub pod: Option<Vec<String>>,
    pub publish: Option<Vec<String>>,
    pub since: Option<Vec<String>>,
    pub status: Option<Vec<ContainerListFiltersStatusOptions>>,
    pub volume: Option<Vec<String>>,
}

pub enum ContainerListFiltersHealthOptions {
    Starting,
    Healthy,
    Unhealthy,
    None,
}

impl ContainerListFiltersHealthOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
            Self::None => "none",
        }
    }
}

pub enum ContainerListFiltersStatusOptions {
    Created,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead,
}

impl ContainerListFiltersStatusOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Restarting => "restarting",
            Self::Running => "running",
            Self::Removing => "removing",
            Self::Paused => "paused",
            Self::Exited => "exited",
            Self::Dead => "dead",
        }
    }
}

pub type ContainerList = Vec<ContainerListItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerListItem {
    pub auto_remove: bool,
    #[serde(rename = "CIDFile")]
    pub cid_file: String,
    pub command: Vec<String>,
    pub created: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub exit_code: i32,
    pub exited: bool,
    pub exited_at: DateTime<Utc>,
    pub exposed_ports: HashMap<u16, Vec<String>>,
    pub id: String,
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    pub is_infra: bool,
    pub labels: HashMap<String, String>,
    pub mounts: Vec<String>,
    pub names: Vec<String>,
    pub namespaces: ContainerListItemNamespaces,
    pub networks: Vec<String>,
    pub pid: i64,
    pub pod: String,
    pub pod_name: String,
    pub ports: Vec<PortMapping>,
    pub restarts: u64,
    pub size: ContainerListItemSize,
    pub started_at: DateTime<Utc>,
    pub state: String,
    pub status: String,
}

impl fmt::Debug for ContainerListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerListItemNamespaces {
    pub cgroup: String,
    pub ipc: String,
    pub mnt: String,
    pub net: String,
    pub pidns: String,
    pub user: String,
    pub uts: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerListItemSize {
    pub root_fs_size: i64,
    pub rw_size: i64,
}
