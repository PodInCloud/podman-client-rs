use std::collections::HashMap;

use http_body_util::BodyExt;
use hyper::{
    Request,
    body::Body,
    client::conn::http1::{self, SendRequest},
    header::HOST,
};
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use tokio::net::UnixStream;

use crate::{
    client::Client,
    models::{lib::Error, podman::error::Error as PodmanError},
};

impl Client {
    pub(crate) async fn build_connection<B>(&self) -> Result<SendRequest<B>, Error>
    where
        B: Body + Send + 'static,
        B::Data: Send,
        B::Error: Into<Error>,
    {
        let stream = UnixStream::connect(self.socket_path()).await?;
        let stream_compat = TokioIo::new(stream);

        let (sender, connection) = http1::handshake::<_, B>(stream_compat).await?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                eprintln!("Connection error: {:?}", err);
            }
        });

        Ok(sender)
    }

    pub(crate) async fn send_request<B, T>(
        &self,
        method: &str,
        path: &str,
        body: B,
    ) -> Result<(HashMap<String, String>, Option<T>), Error>
    where
        B: Body + Send + 'static,
        B::Data: Send,
        B::Error: Into<Error>,
        T: DeserializeOwned,
    {
        let mut sender = self.build_connection().await?;

        let uri = [self.podman_base_url, path].concat();
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header(HOST, "d")
            .body(body)?;

        let res = sender.send_request(req).await?;
        let (res_parts, body) = res.into_parts();
        let res_status = res_parts.status;
        let body_bytes = body.collect().await?.to_bytes();

        if !res_status.is_success() {
            let podman_err = serde_json::from_slice::<PodmanError>(&body_bytes);

            return match podman_err {
                Ok(e) => Err(Box::new(e)),
                Err(_) => Err(format!(
                    "Got error status: {}. Error: {}",
                    res_status,
                    String::from_utf8(body_bytes.to_vec())?,
                )
                .into()),
            };
        }

        let header = res_parts
            .headers
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_owned()))
            .collect();
        let data: Option<T> = if body_bytes.is_empty() {
            None
        } else {
            Some(serde_json::from_slice(&body_bytes)?)
        };

        Ok((header, data))
    }
}
