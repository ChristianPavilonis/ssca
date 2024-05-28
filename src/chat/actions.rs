use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::{Html, IntoResponse},
};
use tokio::sync::broadcast;
use tower_sessions::Session;

use crate::ChatState;
use crate::{chat::ws::handle_socket, rooms::get_room_by_name, AppState};

use super::views::Chat;

pub async fn chat(Path(room): Path<String>, State(state): State<AppState>) -> Html<String> {
    let room = get_room_by_name(&state.db, room).await.unwrap();

    Html(Chat(room).to_string())
}

pub async fn ws(
    session: Session,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(room): Path<String>,
) -> impl IntoResponse {
    let name = match session.get("name").await {
        Ok(Some(name)) => name,
        Ok(None) => panic!("no name"),
        Err(e) => panic!("{e}"),
    };
    let mut rooms = state.chat.rooms.lock().unwrap();

    let tx = match rooms.get(&room) {
        Some(tx) => tx.clone(),
        None => {
            // check if exists in db
            let (tx, _) = broadcast::channel(1000);
            rooms.insert(room, tx.clone());
            tx
        }
    };

    ws.on_upgrade(|socket| handle_socket(socket, tx, name))
}
