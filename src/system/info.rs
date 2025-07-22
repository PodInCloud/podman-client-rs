use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{lib::Error, podman::system::info::SystemInfo},
};

impl Client {
    pub async fn system_info(&self) -> Result<SystemInfo, Error> {
        let (_, data) = self
            .send_request::<_, (), _>("GET", "/libpod/info", Empty::<Bytes>::new())
            .await?;

        Ok(data)
    }
}
