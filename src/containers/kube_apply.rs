use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::containers::kube_apply::{ContainerKubeApply, ContainerKubeApplyOptions},
    },
};

impl Client {
    pub async fn container_kube_apply(
        &self,
        options: ContainerKubeApplyOptions<'_>,
    ) -> Result<ContainerKubeApply, Error> {
        self.pod_kube_apply(options).await
    }
}
