use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Receive a message from this client
            msg = ws_stream.next() => {
                if let Some(Ok(msg)) = msg {
                    if let Some(text) = msg.as_text() {
                        let msg_to_send = format!("{}: {}", addr, text);
                        let _ = bcast_tx.send(msg_to_send);
                    }
                } else {
                    break; // Client disconnected
                }
            }

            // Receive a broadcasted message and send it to this client
            result = bcast_rx.recv() => {
                match result {
                    Ok(msg) => {
                        if ws_stream.send(Message::text(msg)).await.is_err() {
                            break; // Sending failed
                        }
                    }
                    Err(_) => break, // Channel closed
                }
            }
        }
    }

    Ok(())

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}