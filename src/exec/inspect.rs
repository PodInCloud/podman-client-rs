use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::exec::inspect::{ExecInspect, ExecInspectOptions},
    },
};

impl Client {
    pub async fn exec_inspect(
        &self,
        options: ExecInspectOptions<'_>,
    ) -> Result<ExecInspect, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/exec/", options.id, "/json"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
