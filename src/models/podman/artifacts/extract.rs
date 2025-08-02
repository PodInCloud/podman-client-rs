#[derive(Default)]
pub struct ArtifactExtractOptions<'a> {
    pub name: &'a str,
    pub digest: Option<&'a str>,
    pub title: Option<&'a str>,
}
