#[derive(Default)]
pub struct ExecResizeOptions<'a> {
    pub id: &'a str,
    pub h: Option<i64>,
    pub w: Option<i64>,
}
