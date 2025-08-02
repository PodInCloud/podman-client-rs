#[derive(Default)]
pub struct ContainerKillOptions<'a> {
    pub name: &'a str,
    pub signal: Option<&'a str>,
}
