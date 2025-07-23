use std::collections::HashMap;

pub struct SendRequestOptions<'a, RequestBody> {
    pub method: &'a str,
    pub path: &'a str,
    pub header: Option<HashMap<&'a str, &'a str>>,
    pub body: RequestBody,
}
