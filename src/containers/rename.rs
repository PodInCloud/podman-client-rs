use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::containers::rename::ContainerRenameOptions,
    },
};

impl Client {
    pub async fn container_rename(&self, options: ContainerRenameOptions<'_>) -> Result<(), Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &[
                    "/libpod/containers/",
                    options.name,
                    "/rename?name=",
                    options.new_name,
                ]
                .concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
