use podman_client::{client::Client, models::podman::secrets::exists::SecretExistsOptions};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'secret_exists' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let secret_exists = client
        .secret_exists(SecretExistsOptions { name: "my_secret" })
        .await;
    println!("{:#?}", secret_exists);
}
