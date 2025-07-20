use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;

use crate::{client::Client, models::lib::Error};

impl Client {
    pub async fn ping(&self) -> Result<HashMap<String, String>, Error> {
        let (headers, _) = self
            .send_request::<_, ()>("HEAD", "/libpod/_ping", Empty::<Bytes>::new())
            .await?;

        Ok(headers)
    }
}
