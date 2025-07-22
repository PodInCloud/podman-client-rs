use podman_client::{client::Client, models::podman::secrets::inspect::SecretInspectOptions};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'secret_inspect' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let secret_inspect = client
        .secret_inspect(SecretInspectOptions {
            name: "my_secret".to_owned(),
            show_secret: Some(true),
        })
        .await;
    println!("{:#?}", secret_inspect);
}
