#[derive(Default)]
pub struct ContainerCommitOptions<'a> {
    pub author: Option<&'a str>,
    pub changes: Option<Vec<&'a str>>,
    pub comment: Option<&'a str>,
    pub container: &'a str,
    pub format: Option<&'a str>,
    pub pause: Option<bool>,
    pub repo: Option<&'a str>,
    pub squash: Option<bool>,
    pub stream: Option<bool>,
    pub tag: Option<&'a str>,
}
