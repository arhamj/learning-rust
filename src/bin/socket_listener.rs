// src/bin/listener.rs
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // socket closed
                    Ok(n) => {
                        let message = String::from_utf8_lossy(&buf[..n]);
                        println!("Message received: {}", message);
                    }
                    Err(e) => {
                        eprintln!("Failed to receive message: {}", e);
                        return;
                    }
                }
            }
        });
    }
}
