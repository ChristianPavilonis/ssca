#![allow(non_snake_case)]

use crate::layouts::Layout;
use crate::{components::Button, rooms::Room};
use shtml::{html, Component, Render};

pub fn Chat(room: Room) -> Component {
    let ws_url = format!("/chat/ws/{}", room.name);

    html! {
        <Layout>
            <div class="">
                <div id="scroll" class="relative max-h-[100vh] overflow-y-auto pb-60" hx-ext="ws" ws-connect={ws_url}>
                    <div class="flex flex-col gap-12" id="chat_room" hx-swap-oob="beforeend">
                    </div>
                    <form class="fixed bottom-10 flex max-w-600 w-full" id="message-form" ws-send>
                        <input class="bg-gray-700 border border-white rounded text-lg block w-full" name="message">
                        <Button>Send</Button>
                    </form>
                </div>
            </div>

            <script>
                htmx.on("htmx:wsAfterSend", () => {
                    document.querySelector("[name=message]").value = "";
                    document.querySelector("#chat_room");
                });
                htmx.on("htmx:wsAfterMessage", () => {
                    let scroll = document.querySelector("#scroll");
                    scroll.scrollTo(0, scroll.scrollHeight);
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

pub fn JoinedNotification(who: String) -> Component {
    html! {
        <div id="chat_room" hx-swap-oob="beforeend">
            <div class="rounded text-center text-gray-400">
                {format!("{} joined!", who)}
            </div>
        </div>
    }
}
