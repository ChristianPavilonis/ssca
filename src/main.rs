use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};
use chat::actions::{chat, ws};
use components::ButtonLink;
use error::ShatError;
use join::Join;
use layouts::Layout;
use rooms::actions::{create_room, show_create_room, show_rooms};
use shtml::{html, Component, Render};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use state::AppState;
use tokio::time::Duration;
use tower_http::services::ServeDir;
use tower_sessions::{cookie::time, ExpiredDeletion, Expiry, Session, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;
use users::{
    actions::{login, register, show_login, show_register},
    extractors::OptionalUser,
};

mod chat;
mod components;
mod error;
mod join;
mod layouts;
mod rooms;
mod state;
mod users;

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
        .route("/chat/:room", get(chat))
        .route("/chat/ws/:room", get(ws))
        .route("/rooms/create", get(show_create_room))
        .route("/rooms", get(show_rooms))
        .route("/rooms", post(create_room))
        .route("/up", get(health_check))
        .fallback_service(ServeDir::new("public"))
        .with_state(state)
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home(
    session: Session,
    OptionalUser(user): OptionalUser,
) -> Result<Html<String>, ShatError> {
    let name = match session.get("name").await {
        Ok(name) => name,
        Err(_) => return Err(ShatError::InternalError),
    };

    let result = html! {
        <Layout>
            {match user {
                None => html! {
                    <div class="flex justify-end">
                        <button class="underline" hx-get="/login" hx-target="#login">Login</button>
                        <div id="login"></div>
                    </div>
                },
                Some(_) => html!{<></>}
            }}
            <h2 class="text-xl">
                Welcome to the shat stack chat app!
            </h2>
            <Join name=name/>
            <div class="mt-12">
                <ButtonLink href="/rooms">
                    Rooms
                </ButtonLink>
            </div>
        </Layout>
    };

    Ok(Html(result.to_string()))
}

async fn health_check() -> String {
    "all good".to_string()
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
