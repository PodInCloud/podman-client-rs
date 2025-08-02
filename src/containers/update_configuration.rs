use http_body_util::Full;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::update_configuration::{
            ContainerUpdateConfiguration, ContainerUpdateConfigurationOptions,
        },
    },
};

impl Client {
    pub async fn container_update_configuration(
        &self,
        options: ContainerUpdateConfigurationOptions<'_>,
    ) -> Result<ContainerUpdateConfiguration, Error> {
        let mut path = ["/libpod/containers/", options.name, "/update"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(restart_policy) = options.restart_policy {
            query.append_pair("restartPolicy", restart_policy);
        }
        if let Some(restart_retries) = options.restart_retries {
            query.append_pair(
                "restartRetries",
                itoa::Buffer::new().format(restart_retries),
            );
        }
        let query_string = query.finish();
        if !query_string.is_empty() {
            path += &["?", &query_string].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &path,
                header: None,
                body: Full::new(Bytes::from(serde_json::to_string(&options.request)?)),
            })
            .await?;

        Ok(data)
    }
}
