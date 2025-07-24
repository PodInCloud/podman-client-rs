use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::inspect::{ImageInspect, ImageInspectOptions},
    },
};

impl Client {
    pub async fn image_inspect(
        &self,
        options: ImageInspectOptions<'_>,
    ) -> Result<ImageInspect, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/images/", options.name, "/json"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
