use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{connection::SendRequestOptions, lib::Error, podman::system::prune::SystemPrune},
};

impl Client {
    pub async fn system_prune(&self) -> Result<SystemPrune, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: "/libpod/system/prune",
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
