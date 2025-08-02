#[derive(Default)]
pub struct ContainerResizeOptions<'a> {
    pub name: &'a str,
    pub h: Option<i64>,
    pub w: Option<i64>,
}

pub type ContainerResize = serde_json::Value;
