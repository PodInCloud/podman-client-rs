use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct SystemCheckOptions<'a> {
    pub quick: Option<bool>,
    pub repair: Option<bool>,
    pub repair_lossy: Option<bool>,
    pub unreferenced_layer_max_age: Option<&'a str>,
}

#[derive(Deserialize, Serialize)]
pub struct SystemCheck {
    #[serde(rename = "Containers")]
    pub containers: HashMap<String, Vec<String>>,

    #[serde(rename = "Errors")]
    pub errors: bool,

    #[serde(rename = "Images")]
    pub images: HashMap<String, Vec<String>>,

    #[serde(rename = "Layers")]
    pub layers: HashMap<String, Vec<String>>,

    #[serde(rename = "ROImages")]
    pub ro_images: Option<HashMap<String, Vec<String>>>,

    #[serde(rename = "ROLayers")]
    pub ro_layers: Option<HashMap<String, Vec<String>>>,

    #[serde(rename = "RemovedContainers")]
    pub removed_containers: Option<HashMap<String, Vec<String>>>,

    #[serde(rename = "RemovedImages")]
    pub removed_images: Option<HashMap<String, Vec<String>>>,

    #[serde(rename = "RemovedLayers")]
    pub removed_layers: Option<Vec<String>>,
}

impl fmt::Debug for SystemCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
