pub struct Client {
    pub(crate) podman_base_url: &'static str,
    socket_path: String,
}

impl Client {
    pub fn new(socket_path: &str) -> Self {
        Self {
            podman_base_url: "http://d/v5.0.0",
            socket_path: socket_path.to_owned(),
        }
    }

    pub fn socket_path(&self) -> &str {
        &self.socket_path
    }
}
