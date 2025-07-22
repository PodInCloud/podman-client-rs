use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::secrets::inspect::{SecretInspect, SecretInspectOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn secret_inspect(
        &self,
        options: SecretInspectOptions,
    ) -> Result<SecretInspect, Error> {
        let mut path = ["/libpod/secrets/", &options.name, "/json"].concat();

        if let Some(show_secret) = options.show_secret {
            path = [path.as_str(), "?showsecret=", bool_to_str(show_secret)].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>("GET", &path, Empty::<Bytes>::new())
            .await?;

        Ok(data)
    }
}
