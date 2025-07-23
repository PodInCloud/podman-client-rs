use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{connection::SendRequestOptions, lib::Error, podman::system::version::SystemVersion},
};

impl Client {
    pub async fn system_version(&self) -> Result<SystemVersion, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: "/libpod/version",
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
