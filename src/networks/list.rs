use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded::byte_serialize;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::networks::list::{NetworkList, NetworkListOptions},
    },
};

impl Client {
    pub async fn network_list(
        &self,
        options: Option<NetworkListOptions<'_>>,
    ) -> Result<NetworkList, Error> {
        let mut path = "/libpod/networks/json".to_owned();

        if let Some(options) = options {
            if let Some(opt_filters) = options.filters {
                let mut filters = HashMap::new();
                if let Some(name) = opt_filters.name {
                    filters.insert("name", name);
                }
                if let Some(id) = opt_filters.id {
                    filters.insert("id", id);
                }
                if let Some(driver) = opt_filters.driver {
                    filters.insert("driver", driver);
                }
                if let Some(label) = opt_filters.label {
                    filters.insert("label", label);
                }
                if let Some(until) = opt_filters.until {
                    filters.insert("until", until);
                }
                if !filters.is_empty() {
                    let filters_json = serde_json::to_string(&filters)?;
                    let filters_encoded: String = byte_serialize(filters_json.as_bytes()).collect();
                    path += &["?filters=", &filters_encoded].concat();
                }
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
