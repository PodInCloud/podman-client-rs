use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::manifests::inspect::{ManifestInspect, ManifestInspectOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn manifest_inspect(
        &self,
        options: ManifestInspectOptions<'_>,
    ) -> Result<ManifestInspect, Error> {
        let mut path = ["/libpod/manifests/", options.name, "/json"].concat();

        if let Some(tls_verify) = options.tls_verify {
            path += &["?tlsVerify=", bool_to_str(tls_verify)].concat();
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
