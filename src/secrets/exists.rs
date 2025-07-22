use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{lib::Error, podman::secrets::exists::SecretExistsOptions},
};

impl Client {
    pub async fn secret_exists<'a>(&self, options: SecretExistsOptions<'a>) -> Result<(), Error> {
        self.send_request::<_, (), ()>(
            "GET",
            &["/libpod/secrets/", options.name, "/exists"].concat(),
            Empty::<Bytes>::new(),
        )
        .await?;

        Ok(())
    }
}
