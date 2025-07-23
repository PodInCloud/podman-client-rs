use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::system::check::{SystemCheck, SystemCheckOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn system_check(
        &self,
        options: Option<SystemCheckOptions<'_>>,
    ) -> Result<SystemCheck, Error> {
        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(options) = options {
            if let Some(quick) = options.quick {
                query.append_pair("quick", bool_to_str(quick));
            }
            if let Some(repair) = options.repair {
                query.append_pair("repair", bool_to_str(repair));
            }
            if let Some(repair_lossy) = options.repair_lossy {
                query.append_pair("repair_lossy", bool_to_str(repair_lossy));
            }
            if let Some(unreferenced_layer_max_age) = options.unreferenced_layer_max_age {
                query.append_pair("unreferenced_layer_max_age", unreferenced_layer_max_age);
            }
        }

        let query_string = query.finish();

        let mut path = "/libpod/system/check".to_owned();
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
