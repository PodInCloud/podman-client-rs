use podman_client::{client::Client, models::podman::secrets::create_secret::CreateSecretOptions};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'create_secret' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let create_secret = client
        .create_secret(CreateSecretOptions {
            name: "my_secret".to_owned(),
            driver: Some("file".to_owned()),
            driveropts: Some("opt1=val1,opt2=val2".to_owned()),
            labels: Some("env=dev,app=test".to_owned()),
            secret: "secret-value".to_owned(),
        })
        .await;
    println!("{:#?}", create_secret);
}
