use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded::{self, byte_serialize};

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::prune::{ImagePrune, ImagePruneOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn image_prune(
        &self,
        options: Option<ImagePruneOptions<'_>>,
    ) -> Result<ImagePrune, Error> {
        let mut path = "/libpod/images/prune".to_owned();

        if let Some(options) = options {
            let mut query = form_urlencoded::Serializer::new(String::new());
            if let Some(all) = options.all {
                query.append_pair("all", bool_to_str(all));
            }
            if let Some(build_cache) = options.build_cache {
                query.append_pair("buildcache", bool_to_str(build_cache));
            }
            if let Some(external) = options.external {
                query.append_pair("external", bool_to_str(external));
            }
            if let Some(opt_filters) = options.filters {
                let mut filters = HashMap::new();
                if let Some(dangling) = opt_filters.dangling
                    && !dangling.is_empty()
                {
                    filters.insert(
                        "dangling",
                        dangling.iter().map(|&d| bool_to_str(d)).collect(),
                    );
                }
                if let Some(until) = opt_filters.until
                    && !until.is_empty()
                {
                    filters.insert("until", until);
                }
                if let Some(label) = opt_filters.label
                    && !label.is_empty()
                {
                    filters.insert("label", label);
                }
                if let Some(labelnot) = opt_filters.labelnot
                    && !labelnot.is_empty()
                {
                    filters.insert("label!", labelnot);
                }

                if !filters.is_empty() {
                    let filters_json = serde_json::to_string(&filters)?;
                    let filters_encoded: String = byte_serialize(filters_json.as_bytes()).collect();
                    query.append_pair("filters", &filters_encoded);
                }

                let query_string = query.finish();

                if !query_string.is_empty() {
                    path += &["?", &query_string].concat();
                }
            }
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
