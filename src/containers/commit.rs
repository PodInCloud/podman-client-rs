use http_body_util::Empty;
use hyper::body::Bytes;
use url::form_urlencoded;

use crate::{
    client::Client,
    models::{
        connection::SendRequestOptions, lib::Error,
        podman::containers::commit::ContainerCommitOptions,
    },
    utils::bool_to_str::bool_to_str,
};

impl Client {
    pub async fn container_commit(&self, options: ContainerCommitOptions<'_>) -> Result<(), Error> {
        let mut path = "/libpod/commit".to_owned();

        let mut query = form_urlencoded::Serializer::new(String::new());
        query.append_pair("container", options.container);
        if let Some(author) = options.author {
            query.append_pair("author", author);
        }
        if let Some(changes) = options.changes {
            for change in changes {
                query.append_pair("changes", change);
            }
        }
        if let Some(comment) = options.comment {
            query.append_pair("comment", comment);
        }
        if let Some(format) = options.format {
            query.append_pair("format", format);
        }
        if let Some(pause) = options.pause {
            query.append_pair("pause", bool_to_str(pause));
        }
        if let Some(repo) = options.repo {
            query.append_pair("repo", repo);
        }
        if let Some(squash) = options.squash {
            query.append_pair("squash", bool_to_str(squash));
        }
        if let Some(stream) = options.stream {
            query.append_pair("stream", bool_to_str(stream));
        }
        if let Some(tag) = options.tag {
            query.append_pair("tag", tag);
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
