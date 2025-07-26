use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded::{self, byte_serialize};

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::search::{ImageSearch, ImageSearchOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn image_search(
        &self,
        options: Option<ImageSearchOptions<'_>>,
    ) -> Result<ImageSearch, Error> {
        let mut path = "/libpod/images/search".to_owned();

        if let Some(options) = options {
            let mut query = form_urlencoded::Serializer::new(String::new());
            if let Some(opt_filters) = options.filters {
                let mut filters = HashMap::<&str, Vec<String>>::new();
                if let Some(is_automated) = opt_filters.is_automated
                    && !is_automated.is_empty()
                {
                    filters.insert(
                        "is-automated",
                        is_automated
                            .iter()
                            .map(|&b| bool_to_str(b).to_owned())
                            .collect(),
                    );
                }
                if let Some(is_official) = opt_filters.is_official
                    && !is_official.is_empty()
                {
                    filters.insert(
                        "is-official",
                        is_official
                            .iter()
                            .map(|&b| bool_to_str(b).to_owned())
                            .collect(),
                    );
                }
                if let Some(stars) = opt_filters.stars
                    && !stars.is_empty()
                {
                    filters.insert(
                        "stars",
                        stars
                            .iter()
                            .map(|&s| itoa::Buffer::new().format(s).to_owned())
                            .collect(),
                    );
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
            if let Some(list_tags) = options.list_tags {
                query.append_pair("listTags", bool_to_str(list_tags));
            }
            if let Some(term) = options.term {
                query.append_pair("term", term);
            }
            if let Some(tls_verify) = options.tls_verify {
                query.append_pair("tlsVerify", bool_to_str(tls_verify));
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
