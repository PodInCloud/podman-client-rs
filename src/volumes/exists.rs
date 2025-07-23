use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{lib::Error, podman::volumes::exists::VolumeExistsOptions},
};

impl Client {
    pub async fn volume_exists(&self, options: VolumeExistsOptions<'_>) -> Result<(), Error> {
        let (_, data) = self
            .send_request::<_, (), _>(
                "GET",
                &["/libpod/volumes/", options.name, "/exists"].concat(),
                Empty::<Bytes>::new(),
            )
            .await?;

        Ok(data)
    }
}
