use podman_client::client::Client;
use users::get_current_username;

#[tokio::main]
async fn main() {
    println!(
        "Running example 'system_check' on user {:?}",
        get_current_username().unwrap()
    );
    let client = Client::new("/run/user/1000/podman/podman.sock");
    let system_check = client.system_check(None).await;
    println!("{:#?}", system_check);
}
