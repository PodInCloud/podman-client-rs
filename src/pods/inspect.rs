use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::inspect::{PodInspect, PodInspectOptions},
    },
};

impl Client {
    pub async fn pod_inspect(&self, options: PodInspectOptions<'_>) -> Result<PodInspect, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/pods/", options.name, "/json"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
