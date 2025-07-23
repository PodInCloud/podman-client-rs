use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded::byte_serialize;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::secrets::list::{SecretList, SecretListOptions},
    },
};

impl Client {
    pub async fn secret_list(
        &self,
        options: Option<SecretListOptions<'_>>,
    ) -> Result<SecretList, Error> {
        let mut path = "/libpod/secrets/json".to_owned();

        if let Some(options) = options
            && let Some(opt_filters) = options.filters
        {
            let mut filters = HashMap::new();
            if let Some(name) = opt_filters.name
                && !name.is_empty()
            {
                filters.insert("name", name);
            }
            if let Some(id) = opt_filters.id
                && !id.is_empty()
            {
                filters.insert("id", id);
            }

            if !filters.is_empty() {
                let filters_json = serde_json::to_string(&filters)?;
                let filters_encoded: String = byte_serialize(filters_json.as_bytes()).collect();
                path += &["?filters=", &filters_encoded].concat();
            }
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
