use std::{env, net::SocketAddr, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpListener, sync::mpsc};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use super::{Replica, ReplicationError};

pub async fn start_server(
    replica: Arc<Replica>,
    ipaddr: SocketAddr,
) -> Result<(), ReplicationError> {
    let listener = TcpListener::bind(ipaddr).await?;
    println!("Serviço de replicação iniciado: {:?}", ipaddr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                let (peer_tx, mut peer_rx) = mpsc::channel::<String>(10);
                let (mut writer, mut reader) = ws_stream.split();

                tokio::spawn(async move {
                    while let Some(message) = peer_rx.recv().await {
                        if let Ok(response) = serde_json::to_string(&message) {
                            if writer.send(Message::text(response)).await.is_err() {
                                break;
                            }
                        }
                    }
                });

                while let Some(Ok(message)) = reader.next().await {
                    if let Ok(text) = message.to_text() {
                        println!("{:?}", text);
                        let _ = peer_tx.send(text.to_string()).await;
                    } else {
                        eprintln!("Failed to read message: {:?}", message);
                        continue;
                    }
                }

                println!("Disconnected!!!")
            }
        });
    }

    Ok(())
}
