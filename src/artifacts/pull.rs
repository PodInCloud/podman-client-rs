use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::artifacts::pull::{ArtifactPull, ArtifactPullOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn artifact_pull(
        &self,
        options: ArtifactPullOptions<'_>,
    ) -> Result<ArtifactPull, Error> {
        let mut path = "/libpod/artifacts/pull".to_owned();

        let mut query = form_urlencoded::Serializer::new(String::new());
        query.append_pair("name", options.name);
        if let Some(retry) = options.retry {
            query.append_pair("retry", itoa::Buffer::new().format(retry));
        }
        if let Some(retry_delay) = options.retry_delay {
            query.append_pair("retryDelay", retry_delay);
        }
        if let Some(tls_verify) = options.tls_verify {
            query.append_pair("tlsVerify", bool_to_str(tls_verify));
        }
        let query_string = query.finish();
        if !query_string.is_empty() {
            path += &["?", &query_string].concat();
        }

        let mut header = None;
        if let Some(x_registry_auth) = options.x_registry_auth {
            header = Some(HashMap::from([("X-Registry-Auth", x_registry_auth)]));
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &path,
                header,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
