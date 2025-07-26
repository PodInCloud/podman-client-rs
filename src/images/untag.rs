use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error, podman::images::untag::ImageUntagOptions,
    },
};

impl Client {
    pub async fn image_untag(&self, options: ImageUntagOptions<'_>) -> Result<(), Error> {
        let mut path = ["/libpod/images/", options.name, "/untag"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(repo) = options.repo {
            query.append_pair("repo", repo);
        }
        if let Some(tag) = options.tag {
            query.append_pair("tag", tag);
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
