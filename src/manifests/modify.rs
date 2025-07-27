use http_body_util::Full;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::manifests::modify::{ManifestModify, ManifestModifyOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn manifest_modify(
        &self,
        options: ManifestModifyOptions<'_>,
    ) -> Result<ManifestModify, Error> {
        let mut path = ["/libpod/manifests/", options.name].concat();

        if let Some(tls_verify) = options.tls_verify {
            path += &["?tlsVerify=", bool_to_str(tls_verify)].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "PUT",
                path: &path,
                header: None,
                body: Full::new(Bytes::from(serde_json::to_string(&options.request)?)),
            })
            .await?;

        Ok(data)
    }
}
