use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct PodStatsOptions<'a> {
    pub all: Option<bool>,
    pub names_or_ids: Option<Vec<&'a str>>,
}

pub type PodStats = Vec<PodStatsItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodStatsItem {
    #[serde(rename = "BlockIO")]
    pub block_io: String,
    #[serde(rename = "CID")]
    pub cid: String,
    #[serde(rename = "CPU")]
    pub cpu: String,
    pub mem: String,
    pub mem_usage: String,
    pub mem_usage_bytes: String,
    pub name: String,
    #[serde(rename = "NetIO")]
    pub net_io: String,
    #[serde(rename = "PIDS")]
    pub pids: String,
    pub pod: String,
}

impl fmt::Debug for PodStatsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
