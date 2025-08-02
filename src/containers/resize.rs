use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::resize::{ContainerResize, ContainerResizeOptions},
    },
};

impl Client {
    pub async fn container_resize(
        &self,
        options: ContainerResizeOptions<'_>,
    ) -> Result<ContainerResize, Error> {
        let mut path = ["/libpod/containers/", options.name, "/resize"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(h) = options.h {
            query.append_pair("h", itoa::Buffer::new().format(h));
        }
        if let Some(w) = options.w {
            query.append_pair("w", itoa::Buffer::new().format(w));
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
