use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions,
        lib::Error,
        podman::images::change_report::{ImageChangeReport, ImageChangeReportOptions},
    },
};

impl Client {
    pub async fn image_change_report(
        &self,
        options: ImageChangeReportOptions<'_>,
    ) -> Result<ImageChangeReport, Error> {
        let mut path = ["/libpod/images", options.name, "/changes"].concat();

        let mut query = form_urlencoded::Serializer::new(String::new());
        if let Some(diff_type) = options.diff_type {
            query.append_pair("diffType", &serde_json::to_string(&diff_type)?);
        }
        if let Some(parent) = options.parent {
            query.append_pair("parent", parent);
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
