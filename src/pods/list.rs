use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded::byte_serialize;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::list::{PodList, PodListOptions},
    },
};

impl Client {
    pub async fn pod_list(&self, options: Option<PodListOptions<'_>>) -> Result<PodList, Error> {
        let mut path = "/libpod/pods/json".to_owned();

        if let Some(options) = options {
            if let Some(opt_filters) = options.filters {
                let mut filters = HashMap::new();
                if let Some(id) = opt_filters.id {
                    filters.insert("id", id);
                }
                if let Some(label) = opt_filters.label {
                    filters.insert("label", label);
                }
                if let Some(name) = opt_filters.name {
                    filters.insert("name", name);
                }
                if let Some(until) = opt_filters.until {
                    filters.insert("until", until);
                }
                if let Some(status) = opt_filters.status {
                    filters.insert("status", status.iter().map(|s| s.as_str()).collect());
                }
                if let Some(network) = opt_filters.network {
                    filters.insert("network", network);
                }
                if let Some(ctr_names) = opt_filters.ctr_names {
                    filters.insert("ctr-names", ctr_names);
                }
                if let Some(ctr_ids) = opt_filters.ctr_ids {
                    filters.insert("ctr-ids", ctr_ids);
                }
                if let Some(ctr_status) = opt_filters.ctr_status {
                    filters.insert("ctr-status", ctr_status);
                }
                if let Some(ctr_number) = opt_filters.ctr_number {
                    filters.insert("ctr-number", ctr_number);
                }
                if !filters.is_empty() {
                    let filters_json = serde_json::to_string(&filters)?;
                    let filters_encoded: String = byte_serialize(filters_json.as_bytes()).collect();
                    path += &["?filters", &filters_encoded].concat();
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
