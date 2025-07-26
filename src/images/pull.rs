use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::pull::{ImagePull, ImagePullOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn image_pull(
        &self,
        options: Option<ImagePullOptions<'_>>,
    ) -> Result<ImagePull, Error> {
        let mut path = "/libpod/images/pull".to_owned();
        let mut header = None;

        if let Some(options) = options {
            let mut query = form_urlencoded::Serializer::new(String::new());
            if let Some(all_tags) = options.all_tags {
                query.append_pair("allTags", bool_to_str(all_tags));
            }
            if let Some(arch) = options.arch {
                query.append_pair("Arch", arch);
            }
            if let Some(compat_mode) = options.compat_mode {
                query.append_pair("compatMode", bool_to_str(compat_mode));
            }
            if let Some(os) = options.os {
                query.append_pair("OS", os);
            }
            if let Some(policy) = options.policy {
                query.append_pair("policy", policy);
            }
            if let Some(quite) = options.quite {
                query.append_pair("quite", bool_to_str(quite));
            }
            if let Some(reference) = options.reference {
                query.append_pair("reference", reference);
            }
            if let Some(tls_verify) = options.tls_verify {
                query.append_pair("tlsVerify", bool_to_str(tls_verify));
            }
            if let Some(variant) = options.variant {
                query.append_pair("variant", variant);
            }
            let query_string = query.finish();

            if !query_string.is_empty() {
                path += &["?", &query_string].concat();
            }

            if let Some(x_registry_auth) = options.x_registry_auth {
                let mut h = HashMap::new();
                h.insert("x_registry_auth", x_registry_auth);
                header = Some(h);
            }
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &path,
                header,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
