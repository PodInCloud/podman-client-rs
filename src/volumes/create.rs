use http_body_util::Full;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::volumes::create::{VolumeCreate, VolumeCreateOptions},
    },
};

impl Client {
    pub async fn volume_create<'a>(
        &self,
        options: VolumeCreateOptions<'a>,
    ) -> Result<VolumeCreate, Error> {
        let req_body = serde_json::to_vec(&options)?;

        let (_, data) = self
            .send_request::<_, (), _>(
                "POST",
                "/libpod/volumes/create",
                Full::new(Bytes::from(req_body)),
            )
            .await?;

        Ok(data)
    }
}
