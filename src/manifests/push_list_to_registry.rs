use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::manifests::push_list_to_registry::{
            ManifestPushListToRegistry, ManifestPushListToRegistryOptions,
        },
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn manifest_push_list_to_registry(
        &self,
        options: ManifestPushListToRegistryOptions<'_>,
    ) -> Result<ManifestPushListToRegistry, Error> {
        let mut path = [
            "/libpod/manifests/",
            options.name,
            "/registry/",
            options.destination,
        ]
        .concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(add_compression) = options.add_compression {
            for compression in add_compression {
                query.append_pair("addCompression", compression);
            }
        }
        if let Some(all) = options.all {
            query.append_pair("all", bool_to_str(all));
        }
        if let Some(force_compression_format) = options.force_compression_format {
            query.append_pair(
                "forceCompressionFormat",
                bool_to_str(force_compression_format),
            );
        }
        if let Some(quiet) = options.quiet {
            query.append_pair("quiet", bool_to_str(quiet));
        }
        if let Some(tls_verify) = options.tls_verify {
            query.append_pair("tlsVerify", bool_to_str(tls_verify));
        }
        let query_string = query.finish();
        if !query_string.is_empty() {
            path += &["?", &query_string].concat();
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
