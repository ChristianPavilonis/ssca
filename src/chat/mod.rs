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

pub mod actions;
pub mod views;
pub mod ws;

#[derive(Deserialize)]
pub struct Person {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct HtmxMessage {
    message: String,
}

