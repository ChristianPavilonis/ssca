#![allow(non_snake_case)]

use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use shtml::{html as view, Component, Elements, Render};

use crate::ChatState;

#[derive(Deserialize)]
pub struct JoinForm {
    pub name: String,
}

pub async fn join(State(chat): State<ChatState>, Form(join_form): Form<JoinForm>) -> Redirect {
    let mut users = chat.users.lock().unwrap();

    if users.contains(&join_form.name) {
        return Redirect::to("/");
    }

    users.insert(join_form.name.clone());
    let url = format!("/chat?name={}", join_form.name);

    Redirect::to(url.as_str())
}

pub fn Join() -> Component {
    view! {
        <form action="/join" method="post">
            <label>
                <span class="block mb-12 text-lg">Name</span>
                <input class="block rounded text-black" type="text" name="name"/>
            </label>

            <button type="submit">Join</button>
        </form>
    }
}
