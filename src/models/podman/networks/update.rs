use serde::Serialize;

pub struct NetworkUpdateOptions<'a> {
    pub name: &'a str,
    pub request: NetworkUpdateRequest,
}

#[derive(Serialize)]
pub struct NetworkUpdateRequest {
    #[serde(rename = "adddnsservers")]
    pub add_dns_servers: Vec<String>,
    #[serde(rename = "removednsservers")]
    pub remove_dns_servers: Vec<String>,
}
