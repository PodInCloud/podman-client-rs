use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::containers::kube_play_remove::{
            ContainerKubePlayRemove, ContainerKubePlayRemoveOptions,
        },
    },
};

impl Client {
    pub async fn container_kube_play_remove(
        &self,
        options: Option<ContainerKubePlayRemoveOptions>,
    ) -> Result<ContainerKubePlayRemove, Error> {
        self.pod_kube_play_remove(options).await
    }
}
