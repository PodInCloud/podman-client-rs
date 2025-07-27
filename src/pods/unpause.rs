use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::unpause::{PodUnpause, PodUnpauseOptions},
    },
};

impl Client {
    pub async fn pod_unpause(&self, options: PodUnpauseOptions<'_>) -> Result<PodUnpause, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/pods/", options.name, "/unpause"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
