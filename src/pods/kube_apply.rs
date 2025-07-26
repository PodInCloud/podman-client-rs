use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::kube_apply::{PodKubeApply, PodKubeApplyOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn pod_kube_apply(
        &self,
        options: PodKubeApplyOptions<'_>,
    ) -> Result<PodKubeApply, Error> {
        let mut path = "/libpod/kube/apply".to_owned();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(ca_cert_file) = options.ca_cert_file {
            query.append_pair("caCertFile", ca_cert_file);
        }
        if let Some(file) = options.file {
            query.append_pair("file", file);
        }
        if let Some(kube_config) = options.kube_config {
            query.append_pair("kubeConfig", kube_config);
        }
        if let Some(namespace) = options.namespace {
            query.append_pair("namespace", namespace);
        }
        if let Some(service) = options.service {
            query.append_pair("service", bool_to_str(service));
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
                body: options.kubernetes_yaml_file,
            })
            .await?;

        Ok(data)
    }
}
