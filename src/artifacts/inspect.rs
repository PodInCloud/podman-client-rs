use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::artifacts::inspect::{ArtifactInspect, ArtifactInspectOptions},
    },
};

impl Client {
    pub async fn artifact_inspect(
        &self,
        options: ArtifactInspectOptions<'_>,
    ) -> Result<ArtifactInspect, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/artifacts/", options.name, "/json"].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
