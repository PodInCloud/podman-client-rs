use podman_client::{
    client::Client,
    models::podman::volumes::list::{VolumeListFiltersOptions, VolumeListOptions},
};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'volume_list' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let volume_list = client
        .volume_list(Some(VolumeListOptions {
            filters: Some(VolumeListFiltersOptions {
                driver: Some(vec!["local"]),
                ..Default::default()
            }),
        }))
        .await;
    println!("{:#?}", volume_list);
}
