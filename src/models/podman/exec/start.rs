use serde::Serialize;

pub struct ExecStartOptions<'a> {
    pub id: &'a str,
    pub request: ExecStartRequest,
}

#[derive(Serialize)]
pub struct ExecStartRequest {
    #[serde(rename = "Detach")]
    pub detach: bool,
    pub h: i64,
    #[serde(rename = "Tty")]
    pub tty: bool,
    pub w: i64,
}
