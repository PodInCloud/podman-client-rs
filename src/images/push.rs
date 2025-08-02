use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::push::{ImagePush, ImagePushOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn image_push(&self, options: ImagePushOptions<'_>) -> Result<ImagePush, Error> {
        let mut path = ["/libpod/images/", options.name, "/push"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(all) = options.all {
            query.append_pair("all", bool_to_str(all));
        }
        if let Some(compression_format) = options.compression_format {
            query.append_pair("compressionFormat", compression_format);
        }
        if let Some(compression_level) = options.compression_level {
            query.append_pair(
                "compressionLevel",
                itoa::Buffer::new().format(compression_level),
            );
        }
        if let Some(destination) = options.destination {
            query.append_pair("destination", destination);
        }
        if let Some(force_compression_format) = options.force_compression_format {
            query.append_pair(
                "forceCompressionFormat",
                bool_to_str(force_compression_format),
            );
        }
        if let Some(format) = options.format {
            query.append_pair("format", format);
        }
        if let Some(quiet) = options.quiet {
            query.append_pair("quiet", bool_to_str(quiet));
        }
        if let Some(remove_signatures) = options.remove_signatures {
            query.append_pair("removeSignatures", bool_to_str(remove_signatures));
        }
        if let Some(retry) = options.retry {
            query.append_pair("retry", itoa::Buffer::new().format(retry));
        }
        if let Some(retry_delay) = options.retry_delay {
            query.append_pair("retryDelay", retry_delay);
        }
        if let Some(tls_verify) = options.tls_verify {
            query.append_pair("tlsVerify", bool_to_str(tls_verify));
        }
        let query_string = query.finish();
        if !query_string.is_empty() {
            path += &["?", &query_string].concat();
        }

        let mut header = None;
        if let Some(x_registry_auth) = options.x_registry_auth {
            header = Some(HashMap::from([("X-Registry-Auth", x_registry_auth)]));
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
