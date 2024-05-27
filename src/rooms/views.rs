#![allow(non_snake_case)]

use crate::components::Button;
use crate::components::Field;
use crate::layouts::Layout;
use shtml::{html, Component, Render};

use super::Room;

pub fn CreateRoom() -> Component {
    html! {
        <Layout>
            <form method="post" action="/rooms">
                <Field name="name" typ="text">
                    Room name
                </Field>
                <Button>
                    Save
                </Button>
            </form>
        </Layout>
    }
}

pub fn RoomList(rooms: Vec<Room>) -> Component {
    html! {
        <Layout>
            <ul class="space-y-12 mb-12">
            {rooms.iter().map(|room| {
                html!{
                    <li class="bg-gray-600 text-lg p-12 rounded">
                        #{&room.name}
                    </li>
                }
            }).collect::<Vec<_>>()}
            </ul>
            <div class="flex justify-end">
                <a class="rounded bg-gray-700 px-12" href="/rooms/create">Create Room</a>
            </div>
        </Layout>
    }
}
