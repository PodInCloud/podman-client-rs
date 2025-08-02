use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::containers::start::ContainerStartOptions,
    },
};

impl Client {
    pub async fn container_start(&self, options: ContainerStartOptions<'_>) -> Result<(), Error> {
        let mut path = ["/libpod/containers/", options.name, "/start"].concat();

        if let Some(detach_keys) = options.detach_keys {
            path += &["?detachKeys=", detach_keys].concat();
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
