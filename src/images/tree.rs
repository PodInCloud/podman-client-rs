use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::tree::{ImageTree, ImageTreeOptions},
    },
};

impl Client {
    pub async fn image_tree(&self, options: ImageTreeOptions<'_>) -> Result<ImageTree, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/images/", options.name, "/tree"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
