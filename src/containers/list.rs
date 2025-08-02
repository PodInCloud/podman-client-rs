use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded::{self, byte_serialize};

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::list::{ContainerList, ContainerListOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn container_list(
        &self,
        options: Option<ContainerListOptions>,
    ) -> Result<ContainerList, Error> {
        let mut path = "/libpod/containers/json".to_owned();

        if let Some(options) = options {
            let mut query = form_urlencoded::Serializer::new(String::new());
            if let Some(all) = options.all {
                query.append_pair("alla", bool_to_str(all));
            }
            if let Some(opt_filters) = options.filters {
                let mut filters = HashMap::new();
                if let Some(ancestor) = opt_filters.ancestor {
                    filters.insert("ancestor", ancestor);
                }
                if let Some(before) = opt_filters.before {
                    filters.insert("before", before);
                }
                if let Some(expose) = opt_filters.expose {
                    filters.insert("expose", expose);
                }
                if let Some(exited) = opt_filters.exited {
                    filters.insert(
                        "exited",
                        exited
                            .iter()
                            .map(|&d| itoa::Buffer::new().format(d).to_owned())
                            .collect(),
                    );
                }
                if let Some(health) = opt_filters.health {
                    filters.insert(
                        "health",
                        health.iter().map(|h| h.as_str().to_owned()).collect(),
                    );
                }
                if let Some(id) = opt_filters.id {
                    filters.insert("id", id);
                }
                if let Some(is_task) = opt_filters.is_task {
                    filters.insert(
                        "is-task",
                        is_task.iter().map(|&t| bool_to_str(t).to_owned()).collect(),
                    );
                }
                if let Some(label) = opt_filters.label {
                    filters.insert("label", label);
                }
                if let Some(name) = opt_filters.name {
                    filters.insert("name", name);
                }
                if let Some(network) = opt_filters.network {
                    filters.insert("network", network);
                }
                if let Some(pod) = opt_filters.pod {
                    filters.insert("pod", pod);
                }
                if let Some(publish) = opt_filters.publish {
                    filters.insert("publish", publish);
                }
                if let Some(since) = opt_filters.since {
                    filters.insert("since", since);
                }
                if let Some(status) = opt_filters.status {
                    filters.insert(
                        "status",
                        status.iter().map(|s| s.as_str().to_owned()).collect(),
                    );
                }
                if let Some(volume) = opt_filters.volume {
                    filters.insert("volume", volume);
                }
                if !filters.is_empty() {
                    let filters_json = serde_json::to_string(&filters)?;
                    let filters_encoded: String = byte_serialize(filters_json.as_bytes()).collect();
                    query.append_pair("filters", &filters_encoded);
                }
            }
            if let Some(limit) = options.limit {
                query.append_pair("limit", itoa::Buffer::new().format(limit));
            }
            if let Some(namespace) = options.namespace {
                query.append_pair("namespace", bool_to_str(namespace));
            }
            if let Some(pod) = options.pod {
                query.append_pair("pod", bool_to_str(pod));
            }
            if let Some(size) = options.size {
                query.append_pair("size", bool_to_str(size));
            }
            if let Some(sync) = options.sync {
                query.append_pair("sync", bool_to_str(sync));
            }
            let query_string = query.finish();
            if !query_string.is_empty() {
                path += &["?", &query_string].concat();
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
