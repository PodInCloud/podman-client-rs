use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{lib::Error, podman::system::prune::Prune},
};

impl Client {
    pub async fn prune(&self) -> Result<Prune, Error> {
        let (_, data) = self
            .send_request("POST", "/libpod/system/prune", Empty::<Bytes>::new())
            .await?;

        let data = data.ok_or("Missing response body for prune")?;

        Ok(data)
    }
}
