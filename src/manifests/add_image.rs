use http_body_util::Full;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::manifests::add_image::{ManifestAddImage, ManifestAddImageOptions},
    },
};

impl Client {
    pub async fn manifest_add_image(
        &self,
        options: ManifestAddImageOptions<'_>,
    ) -> Result<ManifestAddImage, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/manifests/", options.name, "/add"].concat(),
                header: None,
                body: Full::new(Bytes::from(serde_json::to_string(&options.request)?)),
            })
            .await?;

        Ok(data)
    }
}
