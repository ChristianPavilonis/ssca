use axum::{
    extract::FromRef,
    response::Html,
    routing::{get, post},
    Router,
};
use chat::{actions::ws, views::chat};
use join::Join;
use layouts::Layout;
use rooms::actions::{create_room, show_create_room, show_rooms};
use shtml::{html, Component, Render};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::{
    sync::broadcast::{self, Sender},
    time::Duration,
};
use tower_http::services::ServeDir;
use tower_sessions::{cookie::time, ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;
use users::actions::{login, register, show_login, show_register};

mod chat;
mod components;
mod extractors;
mod join;
mod layouts;
mod rooms;
mod users;

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
    let session_layer = create_session_layer(db.clone()).await;
    let state = AppState::new(db);

    let app = Router::new()
        .route("/", get(home))
        .route("/register", get(show_register))
        .route("/register", post(register))
        .route("/login", get(show_login))
        .route("/login", post(login))
        .route("/join", post(join::join))
        .route("/chat", get(chat))
        .route("/chat/ws", get(ws))
        .route("/rooms/create", get(show_create_room))
        .route("/rooms", get(show_rooms))
        .route("/rooms", post(create_room))
        .fallback_service(ServeDir::new("public"))
        .with_state(state)
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<String> {
    let result = html! {
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

pub async fn test_db() -> Db {
    let db = SqlitePoolOptions::new()
        .connect(":memory:")
        .await
        .expect("could not connect to sqlite database");
    sqlx::migrate!("db/migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    db
}

async fn create_session_layer(db: Db) -> SessionManagerLayer<SqliteStore> {
    let session_store = SqliteStore::new(db);
    session_store
        .migrate()
        .await
        .expect("failed to migrate session store");

    tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(Duration::from_secs(60)),
    );
    let expiry = Expiry::OnInactivity(time::Duration::days(1));

    SessionManagerLayer::new(session_store).with_expiry(expiry)
}
