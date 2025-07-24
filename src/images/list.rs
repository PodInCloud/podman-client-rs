use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded::{self, byte_serialize};

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::list::{ImageList, ImageListOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn image_list(
        &self,
        options: Option<ImageListOptions<'_>>,
    ) -> Result<ImageList, Error> {
        let mut path = "/libpod/images/json".to_owned();

        if let Some(options) = options {
            let mut query = form_urlencoded::Serializer::new(String::new());
            if let Some(all) = options.all {
                query.append_pair("all", bool_to_str(all));
            }
            if let Some(opt_filters) = options.filters {
                let mut filters = HashMap::new();
                if let Some(before) = opt_filters.before
                    && !before.is_empty()
                {
                    filters.insert("before", before);
                }
                if let Some(dangling) = opt_filters.dangling
                    && !dangling.is_empty()
                {
                    let mut f = Vec::with_capacity(dangling.len());
                    for d in dangling {
                        f.push(bool_to_str(d));
                    }
                    filters.insert("dangling", f);
                }
                if let Some(label) = opt_filters.label
                    && !label.is_empty()
                {
                    filters.insert("label", label);
                }
                if let Some(reference) = opt_filters.reference
                    && !reference.is_empty()
                {
                    filters.insert("reference", reference);
                }
                if let Some(id) = opt_filters.id
                    && !id.is_empty()
                {
                    filters.insert("id", id);
                }
                if let Some(since) = opt_filters.since
                    && !since.is_empty()
                {
                    filters.insert("since", since);
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
