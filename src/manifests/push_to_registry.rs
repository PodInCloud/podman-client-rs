use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::manifests::push_to_registry::{
            ManifestPushToRegistry, ManifestPushToRegistryOptions,
        },
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn manifest_push_to_registry(
        &self,
        options: ManifestPushToRegistryOptions<'_>,
    ) -> Result<ManifestPushToRegistry, Error> {
        let mut path = [
            "/libpod/manifests/",
            options.name,
            "/push?destination=",
            options.destination,
        ]
        .concat();

        if let Some(all) = options.all {
            path += &["&all=", bool_to_str(all)].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
