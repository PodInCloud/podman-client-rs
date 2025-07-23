use http_body_util::Full;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::volumes::create::{VolumeCreate, VolumeCreateOptions},
    },
};

impl Client {
    pub async fn volume_create(
        &self,
        options: VolumeCreateOptions<'_>,
    ) -> Result<VolumeCreate, Error> {
        let req_body = serde_json::to_vec(&options)?;

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: "/libpod/volumes/create",
                header: None,
                body: Full::new(Bytes::from(req_body)),
            })
            .await?;

        Ok(data)
    }
}
