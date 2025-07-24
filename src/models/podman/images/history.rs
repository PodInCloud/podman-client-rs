use serde::{Deserialize, Serialize};

pub struct ImageHistoryOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageHistory {
    pub comment: String,
    pub created: i64,
    pub created_by: String,
    pub id: String,
    pub size: i64,
    pub tags: Vec<String>,
}
