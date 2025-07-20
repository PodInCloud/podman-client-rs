use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{lib::Error, podman::system::info::Info},
};

impl Client {
    pub async fn info(&self) -> Result<Info, Error> {
        let (_, data) = self
            .send_request("GET", "/libpod/info", Empty::<Bytes>::new())
            .await?;

        let data = data.ok_or("Missing response body for info")?;

        Ok(data)
    }
}
