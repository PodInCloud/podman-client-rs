use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::containers::kube_play::{ContainerKubePlay, ContainerKubePlayOptions},
    },
};

impl Client {
    pub async fn container_kube_play(
        &self,
        options: ContainerKubePlayOptions<'_>,
    ) -> Result<ContainerKubePlay, Error> {
        self.pod_kube_play(options).await
    }
}
