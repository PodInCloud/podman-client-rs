use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::remove_many::{ImageRemoveMany, ImageRemoveManyOptions},
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn image_remove_many(
        &self,
        options: Option<ImageRemoveManyOptions<'_>>,
    ) -> Result<ImageRemoveMany, Error> {
        let mut path = "/libpod/images/remove".to_owned();

        if let Some(options) = options {
            let mut query = form_urlencoded::Serializer::new(String::new());
            if let Some(all) = options.all {
                query.append_pair("all", bool_to_str(all));
            }
            if let Some(force) = options.force {
                query.append_pair("force", bool_to_str(force));
            }
            if let Some(ignore) = options.ignore {
                query.append_pair("ignore", bool_to_str(ignore));
            }
            if let Some(images) = options.images {
                for image in images {
                    query.append_pair("images", image);
                }
            }
            if let Some(lookup_manifest) = options.lookup_manifest {
                query.append_pair("lookupManifest", bool_to_str(lookup_manifest));
            }
            let query_string = query.finish();
            if !query_string.is_empty() {
                path += &["?", &query_string].concat();
            }
        }

        let (_, data) = self
            .send_request::<_, (), _>(SendRequestOptions {
                method: "DELETE",
                path: &path,
                header: None,
                body: Empty::<Bytes>::new(),
            })
            .await?;

        Ok(data)
    }
}
