use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use crate::framing::encode_frame;

pub async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    println!("Server listening on port 9000");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut seq: u64 = 1;
            loop {
                let payload = format!("{{\"data\": \"frame {}\"}}", seq).into_bytes();
                let frame = encode_frame(1, 0, 0x01, seq, &payload);
                if socket.write_all(&frame).await.is_err() {
                    println!("Client disconnected");
                    break;
                }
                seq += 1;
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });
    }
}