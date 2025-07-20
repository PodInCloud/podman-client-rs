use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::secrets::create_secret::{CreateSecret, CreateSecretOptions},
    },
};

impl Client {
    pub async fn create_secret(&self, options: CreateSecretOptions) -> Result<CreateSecret, Error> {
        let mut query = form_urlencoded::Serializer::new(String::new());
        query.append_pair("name", &options.name);
        if let Some(driver) = &options.driver {
            query.append_pair("driver", &driver);
        }
        if let Some(driveropts) = &options.driveropts {
            query.append_pair("driveropts", &driveropts);
        }
        if let Some(labels) = &options.labels {
            query.append_pair("labels", &labels);
        }

        let query_string = query.finish();

        let (_, data) = self
            .send_request(
                "POST",
                &format!("/libpod/secrets/create?{}", query_string),
                format!("\"{}\"", options.secret),
            )
            .await?;

        let data = data.ok_or("Missing response body for info")?;

        Ok(data)
    }
}
