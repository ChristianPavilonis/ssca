#![allow(non_snake_case)]

use crate::{
    components::{Button, Field},
    state::ChatState,
};
use axum::{
    extract::{Form, State},
    response::{Html, Redirect},
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
) -> Html<String> {
    session.insert("name", &join_form.name).await.unwrap();
    let mut users = chat.users.lock().unwrap();

    if users.contains(&join_form.name) {
        return Html(
            html! {
                Username is taken!
            }
            .to_string(),
        );
    }

    users.insert(join_form.name.clone());

    let name = Some(join_form.name);

    Html(
        html! {
            <Join name=name/>
        }
        .to_string(),
    )
}

pub fn Join(name: Option<String>) -> Component {
    match name {
        Some(name) => html! {
            {format!("Hello, {}!", name)}
        },
        None => html! {
            <form hx-post="/join">
                <Field name="name" typ="text">
                    Name
                </Field>

                <Button>Join</Button>
            </form>
        },
    }
}
