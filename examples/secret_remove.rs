use podman_client::{client::Client, models::podman::secrets::remove::SecretRemoveOptions};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'secret_remove' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let secret_remove = client
        .secret_remove(SecretRemoveOptions {
            name: "my_secret",
            ..Default::default()
        })
        .await;
    println!("{:#?}", secret_remove);
}
