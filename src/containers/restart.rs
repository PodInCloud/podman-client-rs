use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::containers::restart::ContainerRestartOptions,
    },
};

impl Client {
    pub async fn container_restart(
        &self,
        options: ContainerRestartOptions<'_>,
    ) -> Result<(), Error> {
        let mut path = ["/libpod/containers/", options.name, "/restart"].concat();

        if let Some(t) = options.t {
            path += &["?t=", itoa::Buffer::new().format(t)].concat();
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
