use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::remove::{ImageRemove, ImageRemoveOptions},
    },
};

impl Client {
    pub async fn image_remove(
        &self,
        options: ImageRemoveOptions<'_>,
    ) -> Result<ImageRemove, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "DELETE",
                path: &["/libpod/images/", options.name].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
