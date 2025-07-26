#[derive(Default)]
pub struct ImageUntagOptions<'a> {
    pub name: &'a str,
    pub repo: Option<&'a str>,
    pub tag: Option<&'a str>,
}
