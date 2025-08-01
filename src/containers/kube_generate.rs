use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::containers::kube_generate::{ContainerKubeGenerate, ContainerKubeGenerateOptions},
    },
};

impl Client {
    pub async fn container_kube_generate(
        &self,
        options: ContainerKubeGenerateOptions<'_>,
    ) -> Result<ContainerKubeGenerate, Error> {
        self.pod_kube_generate(options).await
    }
}
