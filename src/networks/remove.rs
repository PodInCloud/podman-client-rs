use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::networks::remove::{NetworkRemove, NetworkRemoveOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn network_remove(
        &self,
        options: NetworkRemoveOptions<'_>,
    ) -> Result<NetworkRemove, Error> {
        let mut path = ["/libpod/networks/", options.name].concat();

        if let Some(force) = options.force {
            path += &["?force=", bool_to_str(force)].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "DELETE",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
