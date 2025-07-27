use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::start::{PodStart, PodStartOptions},
    },
};

impl Client {
    pub async fn pod_start(&self, options: PodStartOptions<'_>) -> Result<PodStart, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/pods/", options.name, "/start"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
