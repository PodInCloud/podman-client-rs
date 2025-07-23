use podman_client::{
    client::Client,
    models::podman::volumes::prune::{VolumePruneFiltersOptions, VolumePruneOptions},
};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'volume_prune' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let volume_prune = client
        .volume_prune(Some(VolumePruneOptions {
            filters: Some(VolumePruneFiltersOptions {
                ..Default::default()
            }),
        }))
        .await;
    println!("{:#?}", volume_prune);
}
