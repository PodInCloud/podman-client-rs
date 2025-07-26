#[derive(Default)]
pub struct ImageTagOptions<'a> {
    pub name: &'a str,
    pub repo: &'a str,
    pub tag: Option<&'a str>,
}
