#[derive(Default)]
pub struct ImagePushOptions<'a> {
    pub name: &'a str,
    pub all: Option<bool>,
    pub compression_format: Option<&'a str>,
    pub compression_level: Option<i64>,
    pub destination: Option<&'a str>,
    pub force_compression_format: Option<bool>,
    pub format: Option<&'a str>,
    pub quiet: Option<bool>,
    pub remove_signatures: Option<bool>,
    pub retry: Option<i64>,
    pub retry_delay: Option<&'a str>,
    pub tls_verify: Option<bool>,
    pub x_registry_auth: Option<&'a str>,
}

pub type ImagePush = String;
