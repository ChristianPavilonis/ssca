use axum::{extract::{State, WebSocketUpgrade}, response::IntoResponse};
use tower_sessions::Session;

use crate::ChatState;
use crate::chat::ws::handle_socket;

pub async fn ws(
    session: Session,
    ws: WebSocketUpgrade,
    State(state): State<ChatState>,
) -> impl IntoResponse {
    let name = match session.get("name").await {
        Ok(Some(name)) => name,
        Ok(None) => panic!("no name"),
        Err(e) => panic!("{e}"),
    };
    ws.on_upgrade(|socket| handle_socket(socket, state, name))
}
