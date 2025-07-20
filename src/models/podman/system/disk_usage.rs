use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiskUsage {
    pub images_size: u64,
    pub images: Vec<DiskUsageImage>,
    pub containers: Vec<DiskUsageContainer>,
    pub volumes: Vec<DiskUsageVolume>,
}

impl fmt::Debug for DiskUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize)]
pub struct DiskUsageImage {
    #[serde(rename = "Repository")]
    pub repository: String,

    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde(rename = "ImageID")]
    pub image_id: String,

    #[serde(rename = "Created")]
    pub created: String,

    #[serde(rename = "Size")]
    pub size: u64,

    #[serde(rename = "SharedSize")]
    pub shared_size: u64,

    #[serde(rename = "UniqueSize")]
    pub unique_size: u64,

    #[serde(rename = "Containers")]
    pub containers: u64,
}

#[derive(Deserialize, Serialize)]
pub struct DiskUsageContainer {
    #[serde(rename = "ContainerID")]
    pub container_id: String,

    #[serde(rename = "Image")]
    pub image: String,

    #[serde(rename = "Command")]
    pub command: Vec<String>,

    #[serde(rename = "LocalVolumes")]
    pub local_volumes: u64,

    #[serde(rename = "Size")]
    pub size: u64,

    #[serde(rename = "RWSize")]
    pub rw_size: u64,

    #[serde(rename = "Created")]
    pub created: String,

    #[serde(rename = "Status")]
    pub status: String,

    #[serde(rename = "Names")]
    pub names: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiskUsageVolume {
    pub links: u64,
    pub reclaimable_size: u64,
    pub size: u64,
    pub volume_name: String,
}
