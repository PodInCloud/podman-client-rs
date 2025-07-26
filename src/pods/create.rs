use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::create::{PodCreate, PodCreateOptions},
    },
};

impl Client {
    pub async fn pod_create(&self, options: PodCreateOptions) -> Result<PodCreate, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: "/libpod/pods/create",
                header: None,
                body: serde_json::to_string(&options)?,
            })
            .await?;

        Ok(data)
    }
}
