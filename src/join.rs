#![allow(non_snake_case)]

use axum::{
    extract::Form,
    response::{Html, Redirect},
};
use shtml::{html as view, Component, Elements, Render};

pub fn JoinForm() -> Component {
    view! {
        <form action="/chat" method="get">
            <label>
                <span>Name</span>
                <input type="text" name="name"/>
            </label>

            <button type="submit">Join</button>
        </form>
    }
}
