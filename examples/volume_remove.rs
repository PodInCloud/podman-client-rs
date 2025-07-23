use podman_client::{client::Client, models::podman::volumes::remove::VolumeRemoveOptions};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'volume_remove' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let volume_remove = client
        .volume_remove(VolumeRemoveOptions {
            name: "3bf7359652e02fec8cbc1c0daf7dc36b9338eff21141a9ed8cfbe35c571a8125",
            ..Default::default()
        })
        .await;
    println!("{:#?}", volume_remove);
}
