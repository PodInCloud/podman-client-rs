use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::volumes::inspect::{VolumeInspect, VolumeInspectOptions},
    },
};

impl Client {
    pub async fn volume_inspect(
        &self,
        options: VolumeInspectOptions<'_>,
    ) -> Result<VolumeInspect, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(
                "GET",
                &["/libpod/volumes/", options.name, "/json"].concat(),
                Empty::<Bytes>::new(),
            )
            .await?;

        Ok(data)
    }
}
