use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::kube_generate::{PodKubeGenerate, PodKubeGenerateOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn pod_kube_generate(
        &self,
        options: PodKubeGenerateOptions<'_>,
    ) -> Result<PodKubeGenerate, Error> {
        let mut query = form_urlencoded::Serializer::new(String::new());
        for name in options.names {
            query.append_pair("names", name);
        }
        if let Some(no_trunc) = options.no_trunc {
            query.append_pair("noTrunc", bool_to_str(no_trunc));
        }
        if let Some(podman_only) = options.podman_only {
            query.append_pair("podmanOnly", bool_to_str(podman_only));
        }
        if let Some(replicas) = options.replicas {
            query.append_pair("replicas", itoa::Buffer::new().format(replicas));
        }
        if let Some(service) = options.service {
            query.append_pair("service", bool_to_str(service));
        }
        if let Some(r#type) = options.r#type {
            query.append_pair("type", r#type);
        }
        let query_string = query.finish();

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &["/libpod/generate/kube?", &query_string].concat(),
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
