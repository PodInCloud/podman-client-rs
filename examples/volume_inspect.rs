use podman_client::{client::Client, models::podman::volumes::inspect::VolumeInspectOptions};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'volume_inspect' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let volume_inspect = client
        .volume_inspect(VolumeInspectOptions {
            name: "5227abe5e99f46944c5fdffdf7385f4a53b7d6981919a25e3ca4693be20c11fb",
        })
        .await;
    println!("{:#?}", volume_inspect);
}
