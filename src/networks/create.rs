use http_body_util::Full;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::networks::create::{NetworkCreate, NetworkCreateOptions},
    },
};

impl Client {
    pub async fn network_create(
        &self,
        options: NetworkCreateOptions,
    ) -> Result<NetworkCreate, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: "/libpod/networks/create",
                header: None,
                body: Full::new(Bytes::from(serde_json::to_string(&options)?)),
            })
            .await?;

        Ok(data)
    }
}
