use std::any::TypeId;

use http_body_util::BodyExt;
use hyper::{
    Request,
    body::Body,
    client::conn::http1::{self, SendRequest},
    header::HOST,
};
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use tokio::net::UnixStream;

use crate::{
    client::Client,
    models::{connection::SendRequestOptions, lib::Error, podman::error::Error as PodmanError},
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

    pub(crate) async fn send_request<RequestBody, ResponseHeader, ResponseBody>(
        &self,
        options: SendRequestOptions<'_, RequestBody>,
    ) -> Result<(ResponseHeader, ResponseBody), Error>
    where
        RequestBody: Body + Send + 'static,
        RequestBody::Data: Send,
        RequestBody::Error: Into<Error>,
        ResponseHeader: DeserializeOwned + 'static,
        ResponseBody: DeserializeOwned + 'static,
    {
        let mut sender = self.build_connection().await?;

        let uri = [self.podman_base_url, options.path].concat();
        let req = Request::builder()
            .method(options.method)
            .uri(uri)
            .header(HOST, "d")
            .body(options.body)?;

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

        let header = if TypeId::of::<ResponseHeader>() == TypeId::of::<()>() {
            serde_json::from_slice(b"null")?
        } else {
            let header: Map<String, Value> = res_parts
                .headers
                .iter()
                .filter_map(|(k, v)| {
                    Some((k.to_string(), Value::String(v.to_str().ok()?.to_string())))
                })
                .collect();
            serde_json::from_value(Value::Object(header))?
        };

        let data = if TypeId::of::<ResponseBody>() == TypeId::of::<()>() || body_bytes.is_empty() {
            serde_json::from_slice(b"null")?
        } else {
            serde_json::from_slice(&body_bytes)?
        };

        Ok((header, data))
    }
}
