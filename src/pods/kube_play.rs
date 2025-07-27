use std::collections::HashMap;

use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::kube_play::{PodKubePlay, PodKubePlayOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn pod_kube_play(
        &self,
        options: PodKubePlayOptions<'_>,
    ) -> Result<PodKubePlay, Error> {
        let mut path = "/libpod/play/kube".to_owned();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(annotation) = options.annotation {
            query.append_pair("annotation", &serde_json::to_string(&annotation)?);
        }
        if let Some(build) = options.build {
            query.append_pair("build", bool_to_str(build));
        }
        if let Some(log_driver) = options.log_driver {
            query.append_pair("logDriver", log_driver);
        }
        if let Some(log_options) = options.log_options {
            for log_option in log_options {
                query.append_pair("logOptions", log_option);
            }
        }
        if let Some(network) = options.network {
            for n in network {
                query.append_pair("network", n);
            }
        }
        if let Some(no_hosts) = options.no_hosts {
            query.append_pair("noHosts", bool_to_str(no_hosts));
        }
        if let Some(no_trunc) = options.no_trunc {
            query.append_pair("noTrunc", bool_to_str(no_trunc));
        }
        if let Some(publish_all_ports) = options.publish_all_ports {
            query.append_pair("publishAllPorts", bool_to_str(publish_all_ports));
        }
        if let Some(publish_ports) = options.publish_ports {
            for publish_port in publish_ports {
                query.append_pair("publishPorts", publish_port);
            }
        }
        if let Some(replace) = options.replace {
            query.append_pair("replace", bool_to_str(replace));
        }
        if let Some(service_container) = options.service_container {
            query.append_pair("serviceContainer", bool_to_str(service_container));
        }
        if let Some(start) = options.start {
            query.append_pair("start", bool_to_str(start));
        }
        if let Some(static_ips) = options.static_ips {
            for static_ip in static_ips {
                query.append_pair("staticIPs", static_ip);
            }
        }
        if let Some(static_macs) = options.static_macs {
            for static_mac in static_macs {
                query.append_pair("staticMACs", static_mac);
            }
        }
        if let Some(tls_verify) = options.tls_verify {
            query.append_pair("tlsVerify", bool_to_str(tls_verify));
        }
        if let Some(userns) = options.userns {
            query.append_pair("userns", userns);
        }
        if let Some(wait) = options.wait {
            query.append_pair("wait", bool_to_str(wait));
        }
        let query_string = query.finish();
        if !query_string.is_empty() {
            path += &["?", &query_string].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "POST",
                path: &path,
                header: Some(HashMap::from([("Content-Type", "plain/text")])),
                body: options.kubernetes_yaml_file,
            })
            .await?;

        Ok(data)
    }
}
