#![allow(non_snake_case)]

use crate::components::Button;
use crate::components::Field;
use crate::layouts::Layout;
use crate::users::User;
use shtml::{html, Component, Render};

use super::Room;

pub fn CreateRoom() -> Component {
    html! {
        <dialog open class="max-w-800 rounded w-full bg-gray-600 text-white p-24">
            <form method="post" action="/rooms" class="w-8/10">
                <Field name="name" typ="text">
                    Room name
                </Field>
                <Button>
                    Save
                </Button>
                <button class="rounded bg-gray-700 px-12" value="cancel" formmethod="dialog">Cancel</button>
            </form>
        </dialog>
    }
}

pub fn RoomList(rooms: Vec<Room>, user: Option<User>) -> Component {
    html! {
        <Layout>
            <ul class="space-y-12 mb-12">
            {rooms.iter().map(|room| {
                let link = format!("/chat/{}", room.name);
                html!{
                    <li class="bg-gray-600 text-lg p-12 rounded">
                        <a href=link>
                            #{&room.name}
                        </a>
                    </li>
                }
            }).collect::<Vec<_>>()}
            </ul>
            {if user.is_some() {
                html! {
                    <div class="flex justify-end">
                        <a class="rounded bg-gray-700 px-12" hx-get="/rooms/create" hx-target="#create-container">
                            Create Room
                        </a>
                    </div>
                    <div id="create-container"></div>
                }
            } else {
                html!{<div></div>}
            }}
        </Layout>
    }
}
