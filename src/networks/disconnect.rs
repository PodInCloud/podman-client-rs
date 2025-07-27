use http_body_util::Full;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::networks::disconnect::NetworkDisconnectOptions,
    },
};

impl Client {
    pub async fn network_disconnect(
        &self,
        options: NetworkDisconnectOptions<'_>,
    ) -> Result<(), Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/networks/", options.name, "/disconnect"].concat(),
                header: None,
                body: Full::new(Bytes::from(serde_json::to_string(&options.request)?)),
            })
            .await?;

        Ok(data)
    }
}
