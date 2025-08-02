use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::containers::unmount::ContainerUnmountOptions,
    },
};

impl Client {
    pub async fn container_unmount(
        &self,
        options: ContainerUnmountOptions<'_>,
    ) -> Result<(), Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/containers/", options.name, "/unmount"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
