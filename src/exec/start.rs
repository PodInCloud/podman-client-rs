use http_body_util::Full;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{connection::SendRequestOptions, lib::Error, podman::exec::start::ExecStartOptions},
};

impl Client {
    pub async fn exec_start(&self, options: ExecStartOptions<'_>) -> Result<(), Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &["/libpod/exec/", options.id, "/start"].concat(),
                header: None,
                body: Full::new(Bytes::from(serde_json::to_string(&options.request)?)),
            })
            .await?;

        Ok(data)
    }
}
