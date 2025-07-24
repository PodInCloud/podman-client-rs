use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::history::{ImageHistory, ImageHistoryOptions},
    },
};

impl Client {
    pub async fn image_history(
        &self,
        options: ImageHistoryOptions<'_>,
    ) -> Result<ImageHistory, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/images/", options.name, "/history"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
