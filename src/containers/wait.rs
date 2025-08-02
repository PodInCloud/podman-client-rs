use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::wait::{ContainerWait, ContainerWaitOptions},
    },
};

impl Client {
    pub async fn container_wait(
        &self,
        options: ContainerWaitOptions<'_>,
    ) -> Result<ContainerWait, Error> {
        let mut path = ["/libpod/containers/", options.name, "/wait"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(condition) = options.condition {
            query.append_pair("condition", condition.as_str());
        }
        if let Some(interval) = options.interval {
            query.append_pair("interval", interval);
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
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
