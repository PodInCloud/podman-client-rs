use core::fmt;

use serde::{Deserialize, Serialize};

pub struct ContainerMountOptions<'a> {
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct ContainerMount {
    pub id: String,
}

impl fmt::Debug for ContainerMount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
