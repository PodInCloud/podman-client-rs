use std::collections::HashMap;

#[derive(Default)]
pub struct PodSystemdUnitsGenerateOptions<'a> {
    pub name: &'a str,
    pub additional_env_variables: Option<Vec<&'a str>>,
    pub after: Option<Vec<&'a str>>,
    pub container_prefix: Option<&'a str>,
    pub new: Option<bool>,
    pub no_header: Option<bool>,
    pub pod_prefix: Option<&'a str>,
    pub requires: Option<Vec<&'a str>>,
    pub restart_policy: Option<&'a str>,
    pub restart_sec: Option<i64>,
    pub separator: Option<&'a str>,
    pub start_timeout: Option<i64>,
    pub stop_timeout: Option<i64>,
    pub use_name: Option<bool>,
    pub wants: Option<Vec<&'a str>>,
}

pub type PodSystemdUnitsGenerate = HashMap<String, String>;
