use serde::{Deserialize, Serialize};

pub struct PodPauseOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodPause {
    pub errs: Vec<String>,
    pub id: String,
}
