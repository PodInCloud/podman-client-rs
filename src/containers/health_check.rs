use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::health_check::{ContainerHealthCheck, ContainerHealthCheckOptions},
    },
};

impl Client {
    pub async fn container_health_check(
        &self,
        options: ContainerHealthCheckOptions<'_>,
    ) -> Result<ContainerHealthCheck, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/containers/", options.name, "/healthcheck"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
