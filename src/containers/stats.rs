use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::containers::stats::{ContainerStats, ContainerStatsOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn container_stats(
        &self,
        options: Option<ContainerStatsOptions<'_>>,
    ) -> Result<ContainerStats, Error> {
        let mut path = "/libpod/containers/stats".to_owned();

        if let Some(options) = options {
            let mut query = form_urlencoded::Serializer::new(String::new());
            if let Some(containers) = options.containers {
                for container in containers {
                    query.append_pair("containers", container);
                }
            }
            if let Some(interval) = options.interval {
                query.append_pair("interval", itoa::Buffer::new().format(interval));
            }
            if let Some(stream) = options.stream {
                query.append_pair("stream", bool_to_str(stream));
            }
            let query_string = query.finish();
            if !query_string.is_empty() {
                path += &["?", &query_string].concat();
            }
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
