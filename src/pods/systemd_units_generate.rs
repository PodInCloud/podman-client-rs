use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::pods::systemd_units_generate::{
            PodSystemdUnitsGenerate, PodSystemdUnitsGenerateOptions,
        },
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn pod_systemd_units_generate(
        &self,
        options: PodSystemdUnitsGenerateOptions<'_>,
    ) -> Result<PodSystemdUnitsGenerate, Error> {
        let mut path = ["/libpod/generate/", options.name, "/systemd"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(additional_env_variables) = options.additional_env_variables {
            for additional_env_variable in additional_env_variables {
                query.append_pair("additionalEnvVariables", additional_env_variable);
            }
        }
        if let Some(after) = options.after {
            for a in after {
                query.append_pair("after", a);
            }
        }
        if let Some(container_prefix) = options.container_prefix {
            query.append_pair("containerPrefix", container_prefix);
        }
        if let Some(new) = options.new {
            query.append_pair("new", bool_to_str(new));
        }
        if let Some(no_header) = options.no_header {
            query.append_pair("noHeader", bool_to_str(no_header));
        }
        if let Some(pod_prefix) = options.pod_prefix {
            query.append_pair("podPrefix", pod_prefix);
        }
        if let Some(requires) = options.requires {
            for require in requires {
                query.append_pair("requires", require);
            }
        }
        if let Some(restart_policy) = options.restart_policy {
            query.append_pair("restartPolicy", restart_policy);
        }
        if let Some(restart_sec) = options.restart_sec {
            query.append_pair("restartSec", itoa::Buffer::new().format(restart_sec));
        }
        if let Some(separator) = options.separator {
            query.append_pair("separator", separator);
        }
        if let Some(start_timeout) = options.start_timeout {
            query.append_pair("startTimeout", itoa::Buffer::new().format(start_timeout));
        }
        if let Some(stop_timeout) = options.stop_timeout {
            query.append_pair("stopTimeout", itoa::Buffer::new().format(stop_timeout));
        }
        if let Some(use_name) = options.use_name {
            query.append_pair("useName", bool_to_str(use_name));
        }
        if let Some(wants) = options.wants {
            for want in wants {
                query.append_pair("wants", want);
            }
        }
        let query_string = query.finish();
        if !query_string.is_empty() {
            path += &["?", &query_string].concat();
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "GET",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
