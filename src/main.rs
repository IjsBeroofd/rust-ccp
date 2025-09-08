mod framing;
mod server;
mod client;

use tokio::signal;

#[tokio::main]
async fn main() {
    // Spawn server and client tasks
    let server_task = tokio::spawn(server::run_server());
    let client_task = tokio::spawn(client::run_client());

    // Wait on either task finishing or a Ctrl-C
    tokio::select! {
        res = server_task => {
            eprintln!("Server exited: {:?}", res);
        }
        res = client_task => {
            eprintln!("Client exited: {:?}", res);
        }
        _ = signal::ctrl_c() => {
            println!("Received Ctrl-C, shutting down...");
        }
    }

    // Optional: give tasks a moment to clean up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    println!("Shutdown complete.");
}