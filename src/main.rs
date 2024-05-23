use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{self, FromRef},
    response::Html,
    routing::{get, post},
    Router,
};
use chat::{components::chat, ws};
use join::Join;
use layouts::Layout;
use rooms::actions::{create_room, show_create_room};
use shtml::{html as view, Component, Elements, Render};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::sync::broadcast::{self, Sender};
use tower_http::services::ServeDir;

mod chat;
mod components;
mod join;
mod layouts;
mod rooms;

#[derive(Debug, Clone)]
pub struct AppState {
    pub chat: ChatState,
    pub db: Db,
}

impl AppState {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            chat: ChatState::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatState {
    pub tx: Sender<String>,
    pub users: Arc<Mutex<HashSet<String>>>,
}

impl ChatState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        let users = Arc::new(Mutex::new(HashSet::new()));

        Self { tx, users }
    }
}
impl FromRef<AppState> for ChatState {
    fn from_ref(input: &AppState) -> Self {
        input.chat.clone()
    }
}

#[tokio::main]
async fn main() {
    let db = setup_db().await;
    let state = AppState::new(db);

    let app = Router::new()
        .route("/", get(home))
        .route("/join", post(join::join))
        .route("/chat", get(chat))
        .route("/chat/ws", get(ws))
        .route("/room/create", get(show_create_room))
        .route("/room", post(create_room))
        .fallback_service(ServeDir::new("public"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<String> {
    let result = view! {
        <Layout>
            <h2 class="text-xl">
                Welcome to the shat stack chat app!
            </h2>
            <Join/>
        </Layout>
    };

    Html(result.to_string())
}

type Db = Pool<Sqlite>;

async fn setup_db() -> Db {
    let db = SqlitePoolOptions::new()
        .connect("./db/db.sqlite")
        .await
        .expect("could not connect to sqlite database");
    sqlx::migrate!("db/migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    db
}
