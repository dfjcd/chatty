use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333").await?;
    println!("[SERVER]: Listening on {}", listener.local_addr().unwrap());

    let (tx, _) = broadcast::channel::<String>(12);

    loop {
        let tx = tx.clone();
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            process(socket, tx).await;
        });
    }
}

async fn process(mut socket: TcpStream, tx: broadcast::Sender<String>) {
    let mut buffer = vec![0; 1024];
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            result = socket.read(&mut buffer) => {
                let len = match result {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                       println!("Unable to parse the message | {e}"); return;
                    }
                };
                buffer.resize(len, 0);

                let data = String::from_utf8_lossy(&buffer).to_string();
                tx.send(data).unwrap();
            }
            msg = rx.recv() => {
                match msg {
                    Ok(message) => {
                        socket.write_all(message.as_bytes()).await.unwrap();
                    },
                    Err(e) => println!("Unable to send a message | {e}")
                }
            }
        }
    }
}
