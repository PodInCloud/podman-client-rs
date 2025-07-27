use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ImageChangeReportOptions<'a> {
    pub name: &'a str,
    pub diff_type: Option<ImageChangeReportDiffTypeOptions>,
    pub parent: Option<&'a str>,
}

pub enum ImageChangeReportDiffTypeOptions {
    All,
    Container,
    Image,
}

impl ImageChangeReportDiffTypeOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Container => "container",
            Self::Image => "image",
        }
    }
}

pub type ImageChangeReport = Vec<ImageChangeReportItem>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageChangeReportItem {
    pub path: String,
    pub kind: u8,
}

impl fmt::Debug for ImageChangeReportItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        f.write_str(&json)
    }
}
