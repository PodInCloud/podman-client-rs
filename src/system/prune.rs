use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{lib::Error, podman::system::prune::SystemPrune},
};

impl Client {
    pub async fn system_prune(&self) -> Result<SystemPrune, Error> {
        let (_, data) = self
            .send_request::<_, (), _>("POST", "/libpod/system/prune", Empty::<Bytes>::new())
            .await?;

        Ok(data)
    }
}
