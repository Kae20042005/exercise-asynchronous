use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // Read message from input
            line = stdin.next_line() => {
                if let Ok(Some(line)) = line {
                    if ws_stream.send(Message::text(line)).await.is_err() {
                        eprintln!("Failed to send message");
                        break;
                    }
                }
            }
            // Read message from server
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("{text}");
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("WebSocket error: {:?}", e);
                        break;
                    }
                    None => {
                        eprintln!("Connection closed");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
