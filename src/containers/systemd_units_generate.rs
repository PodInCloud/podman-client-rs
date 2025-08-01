use crate::{
    client::Client,
    models::{
        lib::Error,
        podman::containers::systemd_units_generate::{
            ContainerSystemdUnitsGenerate, ContainerSystemdUnitsGenerateOptions,
        },
    },
};

impl Client {
    pub async fn container_systemd_units_generate(
        &self,
        options: ContainerSystemdUnitsGenerateOptions<'_>,
    ) -> Result<ContainerSystemdUnitsGenerate, Error> {
        self.pod_systemd_units_generate(options).await
    }
}
