use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::inspect::{ContainerInspect, ContainerInspectOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn container_inspect(
        &self,
        options: ContainerInspectOptions<'_>,
    ) -> Result<ContainerInspect, Error> {
        let mut path = ["/libpod/containers/", options.name, "/json"].concat();

        if let Some(size) = options.size {
            path += &["?size=", bool_to_str(size)].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
