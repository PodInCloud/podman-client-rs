# podman-client

A minimal async client for Podman over a Unix socket, written in Rust using `hyper` and `tokio`.

This crate provides basic Podman API access over a Unix domain socket with lightweight dependencies.

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
podman-client = "0.1"
```

## 🚀 Example

Make sure the Podman socket is running:

```sh
podman system service --time=0 unix:///run/user/1000/podman/podman.sock
```

Then use the client like this:

```rust
use podman_client::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("/run/user/1000/podman/podman.sock");

    match client.info().await {
        Ok(info) => println!("{:#?}", info),
        Err(err) => eprintln!("Failed to get podman info: {}", err),
    }
}
```
