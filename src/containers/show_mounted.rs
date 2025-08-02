use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::containers::show_mounted::ContainerShowMounted,
    },
};

impl Client {
    pub async fn container_show_mounted(&self) -> Result<ContainerShowMounted, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: "/libpod/containers/showmounted",
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
