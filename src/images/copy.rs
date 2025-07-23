use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::copy::{ImageCopy, ImageCopyOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn image_copy(&self, options: ImageCopyOptions<'_>) -> Result<ImageCopy, Error> {
        let mut path = ["/libpod/images/scp/", options.name].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(destination) = options.destination
            && !destination.is_empty()
        {
            query.append_pair("destination", destination);
        }
        if let Some(quite) = options.quite {
            query.append_pair("quite", bool_to_str(quite));
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
