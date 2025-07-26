use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IdMap {
    pub container_id: u32,
    pub host_id: u32,
    pub size: u32,
}
