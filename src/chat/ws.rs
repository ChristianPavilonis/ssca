use crate::chat::views::{JoinedNotification, Message};
use crate::chat::HtmxMessage;
use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast::Sender;

pub async fn handle_socket(socket: WebSocket, tx: Sender<String>, name: String) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = tx.subscribe();

    tx.send(JoinedNotification(name.clone()).to_string())
        .expect("failed to send message");

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(raw_message))) = receiver.next().await {
            let message = match serde_json::from_str::<HtmxMessage>(raw_message.as_str()) {
                Ok(htmx) => format!("{}: {}", name, htmx.message),
                Err(e) => {
                    eprintln!("error parsing message {e}");
                    "".to_string()
                }
            };

            let message = Message(message).to_string();
            tx.send(message).expect("failed to send message");
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _= (&mut send_task) => recv_task.abort(),
        _= (&mut recv_task) => send_task.abort()
    }
}
