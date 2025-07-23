use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ImageCreateOptions<'a> {
    pub all_platforms: Option<bool>,
    pub build_args: Option<&'a str>,
    pub cache_from: Option<&'a str>,
    pub compat_volumes: Option<bool>,
    pub cpu_period: Option<i64>,
    pub cpu_quota: Option<i64>,
    pub cpu_set_cpus: Option<&'a str>,
    pub cpu_shares: Option<i64>,
    pub dockerfile: Option<&'a str>,
    pub extra_hosts: Option<&'a str>,
    pub force_rm: Option<bool>,
    pub http_proxy: Option<bool>,
    pub inherit_labels: Option<bool>,
    pub labels: Option<&'a str>,
    pub layer_label: Option<Vec<&'a str>>,
    pub layers: Option<bool>,
    pub memory: Option<i64>,
    pub mem_swap: Option<i64>,
    pub network_mode: Option<&'a str>,
    pub no_cache: Option<bool>,
    pub no_hosts: Option<bool>,
    pub outputs: Option<&'a str>,
    pub platform: Option<&'a str>,
    pub pull: Option<bool>,
    pub q: Option<bool>,
    pub remote: Option<&'a str>,
    pub rm: Option<bool>,
    pub shm_size: Option<i64>,
    pub squash: Option<bool>,
    pub t: Option<&'a str>,
    pub target: Option<&'a str>,
    pub unset_env: Option<Vec<&'a str>>,
    pub unset_label: Option<Vec<&'a str>>,
    pub volume: Option<Vec<&'a str>>,
    pub content_type: Option<&'a str>,
    pub x_registry_config: Option<&'a str>,
}

#[derive(Deserialize, Serialize)]
pub struct ImageCreate {
    pub stream: String,
}

impl fmt::Debug for ImageCreate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
