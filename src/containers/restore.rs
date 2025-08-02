use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::containers::restore::ContainerRestoreOptions,
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn container_restore(
        &self,
        options: ContainerRestoreOptions<'_>,
    ) -> Result<(), Error> {
        let mut path = ["/libpod/containers/", options.name, "/restore"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(file_locks) = options.file_locks {
            query.append_pair("fileLocks", bool_to_str(file_locks));
        }
        if let Some(ignore_root_fs) = options.ignore_root_fs {
            query.append_pair("ignoreRootFS", bool_to_str(ignore_root_fs));
        }
        if let Some(ignore_static_ip) = options.ignore_static_ip {
            query.append_pair("ignoreStaticIP", bool_to_str(ignore_static_ip));
        }
        if let Some(ignore_static_mac) = options.ignore_static_mac {
            query.append_pair("ignoreStaticMAC", bool_to_str(ignore_static_mac));
        }
        if let Some(ignore_volumes) = options.ignore_volumes {
            query.append_pair("ignore_volumes", bool_to_str(ignore_volumes));
        }
        if let Some(import) = options.import {
            query.append_pair("import", bool_to_str(import));
        }
        if let Some(keep) = options.keep {
            query.append_pair("keep", bool_to_str(keep));
        }
        if let Some(name) = options.name_import {
            query.append_pair("name", name);
        }
        if let Some(pod) = options.pod {
            query.append_pair("pod", pod);
        }
        if let Some(print_stats) = options.print_stats {
            query.append_pair("printStats", bool_to_str(print_stats));
        }
        if let Some(tcp_established) = options.tcp_established {
            query.append_pair("tcpEstablished", bool_to_str(tcp_established));
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
