use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded::byte_serialize;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::volumes::prune::{VolumePrune, VolumePruneOptions},
    },
};

impl Client {
    pub async fn volume_prune(
        &self,
        options: Option<VolumePruneOptions<'_>>,
    ) -> Result<VolumePrune, Error> {
        let mut path = "/libpod/volumes/prune".to_owned();

        if let Some(options) = options
            && let Some(opt_filters) = options.filters
        {
            let mut filters = HashMap::new();
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
                path += &["?filters=", &filters_encoded].concat();
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
