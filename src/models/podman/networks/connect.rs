use std::collections::HashMap;

use serde::Serialize;

pub struct NetworkConnectOptions<'a> {
    pub name: &'a str,
    pub request: NetworkConnectRequest<'a>,
}

#[derive(Serialize)]
pub struct NetworkConnectRequest<'a> {
    pub aliases: Vec<&'a str>,
    pub container: &'a str,
    pub interface_name: &'a str,
    pub options: HashMap<String, String>,
    pub static_ips: Vec<&'a str>,
    pub static_mac: &'a str,
}
