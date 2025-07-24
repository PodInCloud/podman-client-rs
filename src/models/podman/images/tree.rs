use serde::{Deserialize, Serialize};

pub struct ImageTreeOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageTree {
    pub tree: String,
}
