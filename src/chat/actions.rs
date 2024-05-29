use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::{Html, IntoResponse},
};
use tokio::sync::broadcast;
use tower_sessions::Session;

use crate::error::ShatError;
use crate::{chat::ws::handle_socket, rooms::get_room_by_name, AppState};

use super::views::Chat;

pub async fn chat(
    Path(room): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, ShatError> {
    let room = get_room_by_name(&state.db, &room)
        .await
        .map_err(|_| ShatError::NotFound)?;

    Ok(Html(Chat(room).to_string()))
}

pub async fn ws(
    session: Session,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(room): Path<String>,
) -> Result<impl IntoResponse, ShatError> {
    let name = match session.get("name").await {
        Ok(Some(name)) => name,
        Ok(None) => return Err(ShatError::BadRequest),
        Err(_) => return Err(ShatError::InternalError),
    };

    let room = get_room_by_name(&state.db, &room)
        .await
        .map_err(|_| ShatError::NotFound)?;

    let mut rooms = state.chat.rooms.lock().unwrap();

    let tx = match rooms.get(&room.name) {
        Some(tx) => tx.clone(),
        None => {
            let (tx, _) = broadcast::channel(1000);
            rooms.insert(room.name, tx.clone());
            tx
        }
    };

    Ok(ws.on_upgrade(|socket| handle_socket(socket, tx, name)))
}
