
use std::time::Duration;
use tokio::time;
use futures::SinkExt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::StreamExt;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    println!("WebSocket server started and listening on port 8080");
    let listener = TcpListener::bind(addr).await.unwrap();
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_client(stream));
    }
}

async fn handle_client(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut write, mut read) = ws_stream.split();
    println!("New WebSocket connection established");

    // Create a task to periodically send updates
    tokio::spawn(async move {
        let mut score = 0;
        let interval_secs = 20;
        let mut interval = time::interval(Duration::from_secs(interval_secs));
        
        loop {
            interval.tick().await;
            score += 1;
            let message = format!("Goal!: 0 - {} - {}", score, "England");
            println!("{} seconds are over. score now: {}", interval_secs, score);
            if write.send(Message::Text(message)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages (if necessary)
    while let Some(Ok(received_msg)) = read.next().await {
        println!("message that client sent back:\n-------\n{}\n--------", received_msg);
        // In this example, we don't need to handle incoming messages
    }
}

