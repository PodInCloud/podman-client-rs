use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::stats::{PodStats, PodStatsOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn pod_stats(&self, options: Option<PodStatsOptions<'_>>) -> Result<PodStats, Error> {
        let mut path = "/libpod/pods/stats".to_owned();

        if let Some(options) = options {
            let mut query = form_urlencoded::Serializer::new(String::new());
            if let Some(all) = options.all {
                query.append_pair("all", bool_to_str(all));
            }
            if let Some(names_or_ids) = options.names_or_ids {
                for name_or_id in names_or_ids {
                    query.append_pair("namesOrIDs", name_or_id);
                }
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
