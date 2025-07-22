use podman_client::{
    client::Client,
    models::podman::secrets::list::{SecretListFiltersOptions, SecretListOptions},
};
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'secret_list' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let secret_list = client
        .secret_list(Some(SecretListOptions {
            filters: Some(SecretListFiltersOptions {
                name: Some(vec!["my_secret"]),
                ..Default::default()
            }),
        }))
        .await;
    println!("{:#?}", secret_list);
}
