mod models;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use models::ChatUser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, RwLock};

use crate::models::{Request, RequestData, Response, ResponseData};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333").await?;
    println!("[SERVER]: Listening on {}", listener.local_addr().unwrap());

    let (tx, _) = broadcast::channel::<String>(12);

    let users = Arc::new(RwLock::new(HashMap::<SocketAddr, ChatUser>::new()));

    loop {
        let tx = tx.clone();
        let (socket, _) = listener.accept().await?;

        let users = users.clone();

        tokio::spawn(async move {
            process(socket, tx, users).await;
        });
    }
}

async fn process(
    mut socket: TcpStream,
    tx: broadcast::Sender<String>,
    users: Arc<RwLock<HashMap<SocketAddr, ChatUser>>>,
) {
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

                let message: Request = match serde_json::from_str(&String::from_utf8_lossy(&buffer)) {
                    Ok(data) => data,
                    Err(e) => {
                        let err_msg = Response {
                            action: "ERR".to_string(),
                            data: ResponseData::Error(e.to_string())
                        };
                        socket.write_all(serde_json::to_string(&err_msg).unwrap().as_bytes()).await.unwrap();
                        return;
                    }
                };

                match (message.action.as_str(), &message.data) {
                    ("LOGIN", RequestData::Login(u)) => {
                       let mut users = users.write().await;
                       let addr = socket.local_addr().unwrap();

                       if users.contains_key(&addr) {
                        let err_msg = Response {
                            action: "ERR".to_string(),
                            data: ResponseData::Error("User already exists".to_string())
                        };

                        socket.write_all(serde_json::to_string(&err_msg).unwrap().as_bytes()).await.unwrap();
                        return;
                       }

                       users.insert(addr, ChatUser {
                        user_name: u.to_string()
                       });

                    },
                    ("MSG", RequestData::Message(m)) => {
                        //send the message
                        println!("{}", m);
                    }
                    _ => {
                        socket.write_all("Invalid message format".as_bytes()).await.unwrap();
                    }
                }

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
