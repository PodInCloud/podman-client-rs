use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error, podman::volumes::remove::VolumeRemoveOptions,
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn volume_remove(&self, options: VolumeRemoveOptions<'_>) -> Result<(), Error> {
        let mut path = ["/libpod/volumes/", options.name].concat();
        if let Some(force) = options.force {
            path += &["?force=", bool_to_str(force)].concat();
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
