#[derive(Default)]
pub struct SecretRemoveOptions<'a> {
    pub name: &'a str,
    pub all: Option<bool>,
}
