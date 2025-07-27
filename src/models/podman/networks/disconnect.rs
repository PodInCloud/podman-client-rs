use serde::Serialize;

pub struct NetworkDisconnectOptions<'a> {
    pub name: &'a str,
    pub request: NetworkDisconnectRequest,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkDisconnectRequest {
    pub container: String,
    pub force: bool,
}
