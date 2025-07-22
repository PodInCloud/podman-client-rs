use podman_client::{client::Client, models::podman::volumes::create::VolumeCreateOptions};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'volume_create' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let volume_create = client
        .volume_create(VolumeCreateOptions {
            ..Default::default()
        })
        .await;
    println!("{:#?}", volume_create);
}
