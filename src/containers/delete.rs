use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::delete::{ContainerDelete, ContainerDeleteOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn container_delete(
        &self,
        options: ContainerDeleteOptions<'_>,
    ) -> Result<ContainerDelete, Error> {
        let mut path = ["/libpod/containers/", options.name].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(depend) = options.depend {
            query.append_pair("depend", bool_to_str(depend));
        }
        if let Some(force) = options.force {
            query.append_pair("force", bool_to_str(force));
        }
        if let Some(ignore) = options.ignore {
            query.append_pair("ignore", bool_to_str(ignore));
        }
        if let Some(timeout) = options.timeout {
            query.append_pair("timeout", itoa::Buffer::new().format(timeout));
        }
        if let Some(v) = options.v {
            query.append_pair("v", bool_to_str(v));
        }
        let query_string = query.finish();
        if !query_string.is_empty() {
            path += &["?", &query_string].concat();
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
