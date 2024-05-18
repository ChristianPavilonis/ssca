use std::sync::Arc;

use axum::{
    extract,
    response::Html,
    routing::{get, post},
    Router,
};
use chat::{components::chat, ws, ChatState};
use join::JoinForm;
use layouts::Layout;
use shtml::{html as view, Component, Elements, Render};
use tower_http::services::ServeDir;

mod chat;
mod components;
mod join;
mod layouts;

#[tokio::main]
async fn main() {
    let chat_state = Arc::new(ChatState::new());

    let app = Router::new()
        .route("/", get(home))
        .route("/chat", get(chat))
        .route("/chat/ws", get(ws))
        .fallback_service(ServeDir::new("public"))
        .with_state(chat_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<String> {
    let result = view! {
        <Layout>
            <JoinForm/>
        </Layout>
    };

    Html(result.to_string())
}
