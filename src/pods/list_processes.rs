use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::list_processes::{PodListProcesses, PodListProcessesOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn pod_list_processes(
        &self,
        options: PodListProcessesOptions<'_>,
    ) -> Result<PodListProcesses, Error> {
        let mut path = ["/libpod/pods/", options.name, "/top"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(delay) = options.delay {
            query.append_pair("delay", itoa::Buffer::new().format(delay));
        }
        if let Some(ps_args) = options.ps_args {
            query.append_pair("ps_args", ps_args);
        }
        if let Some(stream) = options.stream {
            query.append_pair("stream", bool_to_str(stream));
        }
        let query_string = query.finish();
        if !query_string.is_empty() {
            path += &["?", &query_string].concat();
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
