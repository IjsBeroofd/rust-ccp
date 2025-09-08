mod framing;
mod server;
mod client;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        server::run_server().await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    client::run_client().await;
}