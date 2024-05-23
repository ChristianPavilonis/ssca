use crate::ChatState;
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateRoomForm {
    pub name: String,
}

pub async fn create_room<'a>(
    State(state): State<ChatState>,
    Form(form): Form<CreateRoomForm>,
) -> impl IntoResponse {
    // super::create_room(&state.db, form.name).await.unwrap();

    Redirect::to("/")
}

pub async fn show_create_room() -> Html<String> {
    Html(super::views::CreateRoom().to_string())
}
