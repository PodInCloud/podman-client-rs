use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::resolve::{ImageResolve, ImageResolveOptions},
    },
};

impl Client {
    pub async fn resolve(&self, options: ImageResolveOptions<'_>) -> Result<ImageResolve, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/images/", options.name, "/resolve"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
