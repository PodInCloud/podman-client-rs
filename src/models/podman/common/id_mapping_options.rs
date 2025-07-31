use core::fmt;

use serde::{Deserialize, Serialize};

use crate::models::podman::common::id_map::IdMap;

#[derive(Deserialize, Serialize, Default)]
pub struct IdMappingOptions {
    #[serde(rename = "AutoUserNs")]
    pub auto_user_ns: Option<bool>,

    #[serde(rename = "AutoUserNsOpts")]
    pub auto_user_ns_opts: Option<IdMappingOptionsAutoUserNsOptsOptions>,

    #[serde(rename = "GIDMap")]
    pub gid_map: Option<Vec<IdMap>>,

    #[serde(rename = "HostGIDMapping")]
    pub host_gid_mapping: Option<bool>,

    #[serde(rename = "HostUIDMapping")]
    pub host_uid_mapping: Option<bool>,

    #[serde(rename = "UIDMap")]
    pub uid_map: Option<Vec<IdMap>>,
}

impl fmt::Debug for IdMappingOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct IdMappingOptionsAutoUserNsOptsOptions {
    #[serde(rename = "AdditionalGIDMappings")]
    pub additional_gid_mappings: Option<Vec<IdMap>>,

    #[serde(rename = "AdditionalUIDMappings")]
    pub additional_uid_mappings: Option<Vec<IdMap>>,

    #[serde(rename = "GroupFile")]
    pub group_file: Option<String>,

    #[serde(rename = "InitialSize")]
    pub initial_size: Option<u32>,

    #[serde(rename = "PasswdFile")]
    pub passwd_file: Option<String>,

    #[serde(rename = "Size")]
    pub size: Option<u32>,
}
