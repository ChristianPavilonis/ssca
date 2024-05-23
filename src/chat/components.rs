#![allow(non_snake_case)]

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};

use shtml::{html as view, Component, Elements, Render};

use super::Person;
use crate::layouts::Layout;

pub async fn chat(Query(person): Query<Person>) -> Html<String> {
    let html = view! {
        <Chat person=person></Chat>
    }
    .to_string();

    Html(html)
}

pub fn Chat(person: Person) -> Component {
    let ws_url = format!("/chat/ws?name={}", person.name);

    view! {
        <Layout>
            <div class="relative" hx-ext="ws" ws-connect={ws_url}>
                <div class="flex flex-col gap-12" id="chat_room" hx-swap-oob="beforeend">
                </div>
                <form class="fixed bottom-10 flex max-w-600 w-full" id="message-form" ws-send>
                    <input class="bg-gray-700 border border-white rounded text-lg block w-full" name="message">
                    <button class="ml-12 rounded bg-gray-700 px-12">Send</button>
                </form>
            </div>

            <script>
                htmx.on("htmx:wsAfterMessage", () => {
                    document.querySelector("[name=chat_message]").value = "";
                });
            </script>
        </Layout>
    }
}

pub fn Message(text: String) -> Component {
    view! {
        <div id="chat_room" hx-swap-oob="beforeend">
            <div class="bg-gray-600 p-12 rounded">
                {text}
            </div>
        </div>
    }
}
