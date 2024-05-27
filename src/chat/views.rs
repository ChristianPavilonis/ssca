#![allow(non_snake_case)]

use crate::components::Button;
use crate::layouts::Layout;
use axum::{
    response::{Html},
};
use shtml::{html, Component, Render};

pub async fn chat() -> Html<String> {
    Html(Chat().to_string())
}

pub fn Chat() -> Component {
    let ws_url = "/chat/ws";

    html! {
        <Layout>
            <div class="relative max-h-[100vh] overflow-y-auto" hx-ext="ws" ws-connect={ws_url}>
                <div class="flex flex-col gap-12" id="chat_room" hx-swap-oob="beforeend">
                </div>
                <form class="fixed bottom-10 flex max-w-600 w-full" id="message-form" ws-send>
                    <input class="bg-gray-700 border border-white rounded text-lg block w-full" name="message">
                    <Button>Send</Button>
                </form>
            </div>

            <script>
                htmx.on("htmx:wsAfterMessage", () => {
                    document.querySelector("[name=message]").value = "";

                    document.querySelector("#chat_room");
                });
            </script>
        </Layout>
    }
}

pub fn Message(text: String) -> Component {
    html! {
        <div id="chat_room" hx-swap-oob="beforeend">
            <div class="bg-gray-600 p-12 rounded">
                {text}
            </div>
        </div>
    }
}
