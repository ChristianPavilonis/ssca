use crate::ChatState;
use anyhow::bail;
use axum::response::{Html, IntoResponse};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    response::Redirect,
};
use futures::{sink::SinkExt, stream::StreamExt, FutureExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast::{self, Sender};
use tower_sessions::Session;

pub mod components;

#[derive(Deserialize)]
pub struct Person {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct HtmxMessage {
    message: String,
}

pub async fn ws(
    session: Session,
    ws: WebSocketUpgrade,
    State(state): State<ChatState>,
) -> impl IntoResponse {
    let name = match session.get("name").await {
        Ok(Some(name)) => name,
        Ok(None) => 
            panic!("no name"),
        Err(e) => panic!("{e}"),
    };
    ws.on_upgrade(|socket| handle_socket(socket, state, name))
}

async fn handle_socket(socket: WebSocket, state: ChatState, name: String) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = state.tx.subscribe();

    state
        .tx
        .send(format!("{} joined!", name))
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

            let message = components::Message(message).to_string();
            state.tx.send(message).expect("failed to send message");
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _= (&mut send_task) => recv_task.abort(),
        _= (&mut recv_task) => send_task.abort()
    }
}
