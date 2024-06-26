use crate::{
    users::extractors::{AuthenticatedUser, OptionalUser},
    AppState,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;

use super::{
    views::{CreateRoom, RoomList},
    Room,
};

#[derive(Debug, Deserialize)]
pub struct CreateRoomForm {
    pub name: String,
}

pub async fn create_room(
    AuthenticatedUser(_): AuthenticatedUser,
    State(state): State<AppState>,
    Form(form): Form<CreateRoomForm>,
) -> impl IntoResponse {
    super::create_room(&state.db, form.name.to_lowercase())
        .await
        .unwrap();

    Redirect::to("/rooms")
}

pub async fn show_create_room(AuthenticatedUser(_): AuthenticatedUser) -> Html<String> {
    Html(CreateRoom().to_string())
}

pub async fn show_rooms(
    OptionalUser(user): OptionalUser,
    State(state): State<AppState>,
) -> Html<String> {
    let rooms = sqlx::query_as::<_, Room>("select * from rooms")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Html(RoomList(rooms, user).to_string())
}
