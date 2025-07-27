use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::pause::{PodPause, PodPauseOptions},
    },
};

impl Client {
    pub async fn pod_pause(&self, options: PodPauseOptions<'_>) -> Result<PodPause, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/pods/", options.name, "/pause"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
