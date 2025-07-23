use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error, podman::secrets::exists::SecretExistsOptions,
    },
};

impl Client {
    pub async fn secret_exists(&self, options: SecretExistsOptions<'_>) -> Result<(), Error> {
        self.send_request::<_, (), ()>(SendRequestOptions {
            method: "GET",
            path: &["/libpod/secrets/", options.name, "/exists"].concat(),
            header: None,
            body: Empty::<Bytes>::new(),
        })
        .await?;

        Ok(())
    }
}
