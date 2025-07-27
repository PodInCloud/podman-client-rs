use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::kube_play_remove::{PodKubePlayRemove, PodKubePlayRemoveOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn pod_kube_play_remove(
        &self,
        options: Option<PodKubePlayRemoveOptions>,
    ) -> Result<PodKubePlayRemove, Error> {
        let mut path = "/libpod/play/kube".to_owned();

        if let Some(options) = options {
            if let Some(force) = options.force {
                path += &["?force=", bool_to_str(force)].concat();
            }
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
