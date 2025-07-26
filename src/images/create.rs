use std::collections::HashMap;

use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::create::{ImageCreate, ImageCreateOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn image_create(
        &self,
        options: Option<ImageCreateOptions<'_>>,
    ) -> Result<ImageCreate, Error> {
        let mut path = "/libpod/build".to_owned();
        let mut header = None;

        if let Some(options) = options {
            let mut query = form_urlencoded::Serializer::new(String::new());
            if let Some(all_platforms) = options.all_platforms {
                query.append_pair("allplatforms", bool_to_str(all_platforms));
            }
            if let Some(build_args) = options.build_args {
                query.append_pair("buildargs", build_args);
            }
            if let Some(cache_from) = options.cache_from {
                query.append_pair("cachefrom", cache_from);
            }
            if let Some(compat_volumes) = options.compat_volumes {
                query.append_pair("compatvolumes", bool_to_str(compat_volumes));
            }
            if let Some(cpu_period) = options.cpu_period {
                query.append_pair("cpuperiod", itoa::Buffer::new().format(cpu_period));
            }
            if let Some(cpu_quota) = options.cpu_quota {
                query.append_pair("cpuquota", itoa::Buffer::new().format(cpu_quota));
            }
            if let Some(cpu_set_cpus) = options.cpu_set_cpus {
                query.append_pair("cpusetcpus", cpu_set_cpus);
            }
            if let Some(cpu_shares) = options.cpu_shares {
                query.append_pair("cpushares", itoa::Buffer::new().format(cpu_shares));
            }
            if let Some(dockerfile) = options.dockerfile {
                query.append_pair("dockerfile", dockerfile);
            }
            if let Some(extra_hosts) = options.extra_hosts {
                query.append_pair("extrahosts", extra_hosts);
            }
            if let Some(force_rm) = options.force_rm {
                query.append_pair("forcerm", bool_to_str(force_rm));
            }
            if let Some(http_proxy) = options.http_proxy {
                query.append_pair("httpproxy", bool_to_str(http_proxy));
            }
            if let Some(inherit_labels) = options.inherit_labels {
                query.append_pair("inheritlabels", bool_to_str(inherit_labels));
            }
            if let Some(labels) = options.labels {
                query.append_pair("labels", labels);
            }
            if let Some(layer_labels) = options.layer_label {
                for layer_label in layer_labels {
                    query.append_pair("layerLabel", layer_label);
                }
            }
            if let Some(layers) = options.layers {
                query.append_pair("layers", bool_to_str(layers));
            }
            if let Some(memory) = options.memory {
                query.append_pair("memory", itoa::Buffer::new().format(memory));
            }
            if let Some(mem_swap) = options.mem_swap {
                query.append_pair("memswap", itoa::Buffer::new().format(mem_swap));
            }
            if let Some(network_mode) = options.network_mode {
                query.append_pair("networkmode", network_mode);
            }
            if let Some(no_cache) = options.no_cache {
                query.append_pair("nocache", bool_to_str(no_cache));
            }
            if let Some(no_hosts) = options.no_hosts {
                query.append_pair("nohosts", bool_to_str(no_hosts));
            }
            if let Some(outputs) = options.outputs {
                query.append_pair("outputs", outputs);
            }
            if let Some(platform) = options.platform {
                query.append_pair("platform", platform);
            }
            if let Some(pull) = options.pull {
                query.append_pair("pull", bool_to_str(pull));
            }
            if let Some(q) = options.q {
                query.append_pair("q", bool_to_str(q));
            }
            if let Some(remote) = options.remote {
                query.append_pair("remote", remote);
            }
            if let Some(rm) = options.rm {
                query.append_pair("rm", bool_to_str(rm));
            }
            if let Some(shm_size) = options.shm_size {
                query.append_pair("shmsize", itoa::Buffer::new().format(shm_size));
            }
            if let Some(squash) = options.squash {
                query.append_pair("squash", bool_to_str(squash));
            }
            if let Some(t) = options.t {
                query.append_pair("t", t);
            }
            if let Some(target) = options.target {
                query.append_pair("target", target);
            }
            if let Some(unset_envs) = options.unset_env {
                for unset_env in unset_envs {
                    query.append_pair("unsetenv", unset_env);
                }
            }
            if let Some(unset_labels) = options.unset_label {
                for unset_label in unset_labels {
                    query.append_pair("unsetlabel", unset_label);
                }
            }
            if let Some(volumes) = options.volume {
                for volume in volumes {
                    query.append_pair("volume", volume);
                }
            }
            let query_string = query.finish();
            if !query_string.is_empty() {
                path += &["?", query_string.as_str()].concat();
            }

            let mut h = HashMap::new();
            if let Some(content_type) = options.content_type {
                h.insert("Content-Type", content_type);
            }
            if let Some(x_registry_config) = options.x_registry_config {
                h.insert("X-Registry-Config", x_registry_config);
            }
            if !h.is_empty() {
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
