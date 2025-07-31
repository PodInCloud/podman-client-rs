use http_body_util::Full;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::create::{ContainerCreate, ContainerCreateOptions},
    },
};

impl Client {
    pub async fn container_create(
        &self,
        options: ContainerCreateOptions,
    ) -> Result<ContainerCreate, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: "libpod/containers/create",
                header: None,
                body: Full::new(Bytes::from(serde_json::to_string(&options)?)),
            })
            .await?;

        Ok(data)
    }
}
