use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buff= vec![0; 1024];

            loop {
                match socket.read(&mut buff).await {
                    Ok(0) => return,
                    Ok(n) => {
                        let request = String::from_utf8(Vec::from(&buff[0..n]));
                        match request {
                            Ok(r) => println!("ECHO: {r}"),
                            Err(_) => eprintln!("Error occurred while deserializing the message")
                        }
                    }
                    Err(_) => return
                }
            }
        });
    }
}