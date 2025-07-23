use podman_client::{client::Client, models::podman::volumes::exists::VolumeExistsOptions};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'volume_exists' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let volume_exists = client
        .volume_exists(VolumeExistsOptions {
            name: "5227abe5e99f46944c5fdffdf7385f4a53b7d6981919a25e3ca4693be20c11fb",
        })
        .await;
    println!("{:#?}", volume_exists);
}
