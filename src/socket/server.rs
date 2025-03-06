use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpListener, sync::mpsc};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use super::{Commands, Responses, SocketError};

pub async fn start() -> Result<(), SocketError> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on: 8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                let (peer_tx, mut peer_rx) = mpsc::channel::<Responses>(10);
                let (mut write, mut read) = ws_stream.split();

                // Spawn para enviar resposta a cada conexão
                tokio::spawn(async move {
                    while let Some(message) = peer_rx.recv().await {
                        if let Ok(response) = serde_json::to_string(&message) {
                            if write.send(Message::text(response)).await.is_err() {
                                break;
                            }
                        }
                    }
                });

                // Loop para ler mensagens do cliente
                while let Some(Ok(message)) = read.next().await {
                    if let Ok(text) = message.to_text() {
                        if let Ok(command) = serde_json::from_str::<Commands>(text) {
                            match command {
                                Commands::Test(s) => {
                                    let response = Responses::Test(s);
                                    let _ = peer_tx.send(response).await;
                                }
                            }
                        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_tungstenite::connect_async;

    use crate::socket::start;

    #[tokio::test]
    async fn test_server() {
        tokio::spawn(async {
            let _ = start().await;
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

        // Conectar ao servidor WebSocket
        let url = "ws://127.0.0.1:8080";
        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        let command = serde_json::to_string(&Commands::Test("Hello".into()))
            .expect("Failed to crate command");
        ws_stream
            .send(Message::Text(command.into()))
            .await
            .expect("failed to send command");

        if let Some(Ok(Message::Text(response))) = ws_stream.next().await {
            let parsed: Responses =
                serde_json::from_str(&response).expect("Failed to parse response");
            assert_eq!(
                parsed,
                Responses::Test("Hello".into()),
                "Unexpected response"
            );
        }
    }
}
