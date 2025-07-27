use http_body_util::Full;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::manifests::create::{ManifestCreate, ManifestCreateOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn manifest_create(
        &self,
        options: ManifestCreateOptions<'_>,
    ) -> Result<ManifestCreate, Error> {
        let mut query = form_urlencoded::Serializer::new(String::new());
        for image in options.images {
            query.append_pair("images", image);
        }
        if let Some(all) = options.all {
            query.append_pair("all", bool_to_str(all));
        }
        if let Some(amend) = options.amend {
            query.append_pair("amend", bool_to_str(amend));
        }
        let query_string = query.finish();

        let path = ["/libpod/manifests/", options.name, "?", &query_string].concat();

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &path,
                header: None,
                body: Full::new(Bytes::from(serde_json::to_string(&options.request)?)),
            })
            .await?;

        Ok(data)
    }
}
