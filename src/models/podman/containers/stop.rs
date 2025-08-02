#[derive(Default)]
pub struct ContainerStopOptions<'a> {
    pub name: &'a str,
    pub ignore: Option<bool>,
    pub timeout: Option<i64>,
}
