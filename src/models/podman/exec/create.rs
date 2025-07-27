use serde::Serialize;

pub struct ExecCreateOptions<'a> {
    pub name: &'a str,
    pub request: ExecCreateRequest,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExecCreateRequest {
    pub attach_stderr: bool,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub cmd: Vec<String>,
    pub detach_keys: String,
    pub env: Vec<String>,
    pub privileged: bool,
    pub tty: bool,
    pub user: String,
    pub working_dir: String,
}
