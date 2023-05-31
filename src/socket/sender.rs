use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn start_sender() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";
    let socket = TcpStream::connect(&addr).await?;
    println!("Connected to: {}", addr);
    let (mut reader, mut writer) = socket.into_split();
    let mut buf = [0; 1024];
    loop {
        let n = reader.read(&mut buf).await.unwrap();
        if n == 0 {
            return Ok(());
        }
        writer.write_all(&buf[0..n]).await.unwrap();
    }
}
