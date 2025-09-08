use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

pub async fn run_client() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();
    let mut buf = vec![0u8; 1024];

    loop {
        let n = stream.read(&mut buf).await.unwrap();
        if n == 0 { break; }
        let payload = &buf[16..n]; // skip header
        println!("Received: {}", String::from_utf8_lossy(payload));
    }
}