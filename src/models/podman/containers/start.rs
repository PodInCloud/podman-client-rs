#[derive(Default)]
pub struct ContainerStartOptions<'a> {
    pub name: &'a str,
    pub detach_keys: Option<&'a str>,
}
