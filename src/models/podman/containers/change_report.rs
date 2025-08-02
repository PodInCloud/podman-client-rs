use crate::models::podman::images::change_report::{ImageChangeReport, ImageChangeReportOptions};

pub type ContainerChangeReportOptions<'a> = ImageChangeReportOptions<'a>;

pub type ContainerChangeReport = ImageChangeReport;
