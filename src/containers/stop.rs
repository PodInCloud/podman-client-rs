use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error, podman::containers::stop::ContainerStopOptions,
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn container_stop(&self, options: ContainerStopOptions<'_>) -> Result<(), Error> {
        let mut path = ["/libpod/containers/", options.name, "/stop"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(ignore) = options.ignore {
            query.append_pair("Ignore", bool_to_str(ignore));
        }
        if let Some(timeout) = options.timeout {
            query.append_pair("timeout", itoa::Buffer::new().format(timeout));
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
