use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{connection::SendRequestOptions, lib::Error, podman::images::tag::ImageTagOptions},
};

impl Client {
    pub async fn image_tag(&self, options: ImageTagOptions<'_>) -> Result<(), Error> {
        let mut path = ["/libpod/images/", options.name, "/tag?repo=", options.repo].concat();
        if let Some(tag) = options.tag {
            path += &["tag=", tag].concat();
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
