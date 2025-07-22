use podman_client::{client::Client, models::podman::secrets::create::SecretCreateOptions};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'secret_create' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let secret_create = client
        .secret_create(SecretCreateOptions {
            name: "my_secret",
            driver: Some("file"),
            driver_opts: Some("opt1=val1,opt2=val2"),
            labels: Some("env=dev,app=test"),
            secret: "secret-value",
        })
        .await;
    println!("{:#?}", secret_create);
}
