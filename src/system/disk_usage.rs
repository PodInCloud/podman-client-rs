use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error, podman::system::disk_usage::SystemDiskUsage,
    },
};

impl Client {
    pub async fn system_disk_usage(&self) -> Result<SystemDiskUsage, Error> {
        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: "/libpod/system/df",
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
