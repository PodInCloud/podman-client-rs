use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::secrets::create::{SecretCreate, SecretCreateOptions},
    },
};

impl Client {
    pub async fn secret_create<'a>(
        &self,
        options: SecretCreateOptions<'a>,
    ) -> Result<SecretCreate, Error> {
        let mut query = form_urlencoded::Serializer::new(String::new());
        query.append_pair("name", &options.name);
        if let Some(driver) = &options.driver {
            query.append_pair("driver", &driver);
        }
        if let Some(driver_opts) = &options.driver_opts {
            query.append_pair("driveropts", &driver_opts);
        }
        if let Some(labels) = &options.labels {
            query.append_pair("labels", &labels);
        }

        let query_string = query.finish();

        let (_, data) = self
            .send_request::<_, (), _>(
                "POST",
                &["/libpod/secrets/create?", &query_string].concat(),
                options.secret.to_owned(),
            )
            .await?;

        Ok(data)
    }
}
