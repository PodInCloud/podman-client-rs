use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct PodKillOptions<'a> {
    pub name: &'a str,
    pub signal: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodKill {
    pub errs: Vec<String>,
    pub id: String,
}
