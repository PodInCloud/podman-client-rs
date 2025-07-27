use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::networks::inspect::{NetworkInspect, NetworkInspectOptions},
    },
};

impl Client {
    pub async fn network_inspect(
        &self,
        options: NetworkInspectOptions<'_>,
    ) -> Result<NetworkInspect, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/networks/", options.name, "/json"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
