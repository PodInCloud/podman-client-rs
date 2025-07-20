use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{lib::Error, podman::system::disk_usage::DiskUsage},
};

impl Client {
    pub async fn disk_usage(&self) -> Result<DiskUsage, Error> {
        let (_, data) = self
            .send_request("GET", "/libpod/system/df", Empty::<Bytes>::new())
            .await?;

        let data = data.ok_or("Missing response body for disk usage")?;

        Ok(data)
    }
}
