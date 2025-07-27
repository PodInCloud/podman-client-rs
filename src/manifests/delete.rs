use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::manifests::delete::{ManifestDelete, ManifestDeleteOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn manifest_delete(
        &self,
        options: ManifestDeleteOptions<'_>,
    ) -> Result<ManifestDelete, Error> {
        let mut path = ["/libpod/manifests/", options.name].concat();

        if let Some(ignore) = options.ignore {
            path += &["?ignore=", bool_to_str(ignore)].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "DELETE",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
