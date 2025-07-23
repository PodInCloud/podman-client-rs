use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{connection::SendRequestOptions, lib::Error, podman::system::info::SystemInfo},
};

impl Client {
    pub async fn system_info(&self) -> Result<SystemInfo, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: "/libpod/info",
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
