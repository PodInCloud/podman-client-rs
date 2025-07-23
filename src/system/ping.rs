use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{connection::SendRequestOptions, lib::Error, podman::system::ping::SystemPing},
};

impl Client {
    pub async fn system_ping(&self) -> Result<SystemPing, Error> {
        let (headers, _) = self
            .send_request::<_, _, ()>(SendRequestOptions {
                method: "HEAD",
                path: "/libpod/_ping",
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(headers)
    }
}
