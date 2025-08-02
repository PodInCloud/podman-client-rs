use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::mount::{ContainerMount, ContainerMountOptions},
    },
};

impl Client {
    pub async fn container_mount(
        &self,
        options: ContainerMountOptions<'_>,
    ) -> Result<ContainerMount, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/containers/", options.name, "/mount"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
