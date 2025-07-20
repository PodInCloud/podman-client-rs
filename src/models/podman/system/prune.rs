use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Prune {
    pub pod_prune_report: Option<Vec<PruneReport>>,
    pub container_prune_reports: Option<Vec<PruneReport>>,
    pub image_prune_reports: Option<Vec<PruneReport>>,
    pub network_prune_reports: Option<Vec<PruneReport>>,
    pub volume_prune_reports: Option<Vec<PruneReport>>,
    pub reclaimed_space: u64,
}

impl fmt::Debug for Prune {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum PruneReport {
    Pod(PruneReportPod),
    Network(PruneReportNetwork),
    Else(PruneReportElse),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PruneReportPod {
    pub err: String,
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PruneReportNetwork {
    pub error: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PruneReportElse {
    pub err: String,
    pub id: String,
    pub size: u64,
}
