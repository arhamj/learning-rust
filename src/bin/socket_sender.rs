use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let mut reader = BufReader::new(io::stdin());

    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line).await?;
        if n == 0 {
            break;
        }

        socket.write_all(line.as_bytes()).await?;
    }

    Ok(())
}
