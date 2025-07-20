use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{lib::Error, podman::system::version::Version},
};

impl Client {
    pub async fn version(&self) -> Result<Version, Error> {
        let (_, data) = self
            .send_request("GET", "/libpod/version", Empty::<Bytes>::new())
            .await?;

        let data = data.ok_or("Missing response body for version")?;

        Ok(data)
    }
}
