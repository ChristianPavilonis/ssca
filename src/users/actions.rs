use super::{
    auth::{check_password, register_user},
    find_user_by_name,
    views::{Login, Register},
};
use crate::state::AppState;
use axum::{
    extract::State,
    response::{Html, Redirect},
    Form,
};
use serde::Deserialize;
use tower_sessions::Session;

pub async fn show_register() -> Html<String> {
    Html(Register().to_string())
}

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    pub name: String,
    pub password: String,
}

pub async fn register(State(state): State<AppState>, Form(form): Form<RegisterForm>) -> Redirect {
    match register_user(form.name, form.password, &state.db).await {
        Ok(_) => Redirect::to("/"),
        Err(_) => Redirect::to("/"),
    }
}

pub async fn show_login() -> Html<String> {
    Html(Login().to_string())
}

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub name: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> Redirect {
    match find_user_by_name(&state.db, form.name).await {
        None => panic!("no user found"),
        Some(user) => {
            if check_password(&user, form.password) {
                session.insert("user_id", user.id.unwrap()).await.ok();
                Redirect::to("/")
            } else {
                Redirect::to("/")
            }
        }
    }
}
