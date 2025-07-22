use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{lib::Error, podman::system::ping::SystemPing},
};

impl Client {
    pub async fn system_ping(&self) -> Result<SystemPing, Error> {
        let (headers, _) = self
            .send_request::<_, _, ()>("HEAD", "/libpod/_ping", Empty::<Bytes>::new())
            .await?;

        Ok(headers)
    }
}
