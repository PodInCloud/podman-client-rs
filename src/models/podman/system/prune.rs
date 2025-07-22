use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemPrune {
    pub pod_prune_report: Option<Vec<SystemPruneReportPod>>,
    pub container_prune_reports: Option<Vec<SystemPruneReportElse>>,
    pub image_prune_reports: Option<Vec<SystemPruneReportElse>>,
    pub network_prune_reports: Option<Vec<SystemPruneReportNetwork>>,
    pub volume_prune_reports: Option<Vec<SystemPruneReportElse>>,
    pub reclaimed_space: u64,
}

impl fmt::Debug for SystemPrune {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemPruneReportPod {
    pub err: String,
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemPruneReportNetwork {
    pub error: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemPruneReportElse {
    pub err: String,
    pub id: String,
    pub size: u64,
}
