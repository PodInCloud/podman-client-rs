use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error, podman::secrets::remove::SecretRemoveOptions,
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn secret_remove(&self, options: SecretRemoveOptions<'_>) -> Result<(), Error> {
        let mut path = ["/libpod/secrets/", &options.name].concat();

        if let Some(all) = options.all {
            path = [path.as_str(), "?all=", bool_to_str(all)].concat();
        }

        self.send_request::<_, (), ()>(SendRequestOptions {
            method: "DELETE",
            path: &path,
            header: None,
            body: Empty::<Bytes>::new(),
        })
        .await?;

        Ok(())
    }
}
