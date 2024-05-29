use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use tokio::{
    sync::broadcast::{Sender},
};

use crate::{error::ShatError, Db};

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub chat: ChatState,
    pub db: Db,
}

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ShatError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
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
    pub rooms: Arc<Mutex<HashMap<String, Sender<String>>>>,
    pub users: Arc<Mutex<HashSet<String>>>, // should maybe nest it into rooms
}

impl ChatState {
    pub fn new() -> Self {
        let rooms = Arc::new(Mutex::new(HashMap::new()));
        let users = Arc::new(Mutex::new(HashSet::new()));

        Self { rooms, users }
    }
}
