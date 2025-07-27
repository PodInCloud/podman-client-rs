use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::stop::{PodStop, PodStopOptions},
    },
};

impl Client {
    pub async fn pod_stop(&self, options: PodStopOptions<'_>) -> Result<PodStop, Error> {
        let mut path = ["/libpod/pods/", options.name, "/stop"].concat();

        if let Some(t) = options.t {
            path += &["?t=", itoa::Buffer::new().format(t)].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
