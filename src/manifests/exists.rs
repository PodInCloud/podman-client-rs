use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::manifests::exists::ManifestExistsOptions,
    },
};

impl Client {
    pub async fn manifest_exists(&self, options: ManifestExistsOptions<'_>) -> Result<(), Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/manifests/", options.name, "/exists"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
