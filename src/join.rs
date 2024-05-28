#![allow(non_snake_case)]

use crate::{
    components::{Button, Field},
    state::ChatState,
};
use axum::{
    extract::{Form, State},
    response::Redirect,
};
use serde::Deserialize;
use shtml::{html, Component, Render};
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct JoinForm {
    pub name: String,
}

pub async fn join(
    State(chat): State<ChatState>,
    session: Session,
    Form(join_form): Form<JoinForm>,
) -> Redirect {
    session.insert("name", &join_form.name).await.unwrap();
    let mut users = chat.users.lock().unwrap();

    if users.contains(&join_form.name) {
        return Redirect::to("/");
    }

    users.insert(join_form.name.clone());

    Redirect::to("/chat")
}

pub fn Join() -> Component {
    html! {
        <form action="/join" method="post">
            <Field name="name" typ="text">
                Name
            </Field>

            <Button>Join</Button>
        </form>
    }
}
