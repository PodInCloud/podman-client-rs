use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::artifacts::remove::{ArtifactRemove, ArtifactRemoveOptions},
    },
};

impl Client {
    pub async fn artifact_remove(
        &self,
        options: ArtifactRemoveOptions<'_>,
    ) -> Result<ArtifactRemove, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "DELETE",
                path: &["/libpod/artifacts/", options.name].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
