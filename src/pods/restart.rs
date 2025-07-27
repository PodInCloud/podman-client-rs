use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::restart::{PodRestart, PodRestartOptions},
    },
};

impl Client {
    pub async fn pod_restart(&self, options: PodRestartOptions<'_>) -> Result<PodRestart, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/pods/", options.name, "/restart"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
