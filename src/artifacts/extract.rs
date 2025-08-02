use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::artifacts::extract::ArtifactExtractOptions,
    },
};

impl Client {
    pub async fn artifact_extract(&self, options: ArtifactExtractOptions<'_>) -> Result<(), Error> {
        let mut path = ["/libpod/artifacts/", options.name, "/extract"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(digest) = options.digest {
            query.append_pair("digest", digest);
        }
        if let Some(title) = options.title {
            query.append_pair("title", title);
        }
        let query_string = query.finish();
        if !query_string.is_empty() {
            path += &["?", &query_string].concat();
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
