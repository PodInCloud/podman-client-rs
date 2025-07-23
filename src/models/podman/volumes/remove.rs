#[derive(Default)]
pub struct VolumeRemoveOptions<'a> {
    pub name: &'a str,
    pub force: Option<bool>,
}
