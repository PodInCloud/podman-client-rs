use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::kill::{PodKill, PodKillOptions},
    },
};

impl Client {
    pub async fn pod_kill(&self, options: PodKillOptions<'_>) -> Result<PodKill, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/pods/", options.name, "/kill"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
