use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error, podman::containers::kill::ContainerKillOptions,
    },
};

impl Client {
    pub async fn container_kill(&self, options: ContainerKillOptions<'_>) -> Result<(), Error> {
        let mut path = ["/libpod/containers/", options.name, "/kill"].concat();

        if let Some(signal) = options.signal {
            path += &["?signal=", signal].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
