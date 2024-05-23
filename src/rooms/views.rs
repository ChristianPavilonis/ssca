#![allow(non_snake_case)]

use crate::layouts::Layout;
use shtml::{html as view, Component, Render};

pub fn CreateRoom() -> Component {
    view! {
        <Layout>
            <form method="post" action="/room">
                <label>
                    Room name
                    <input type="text" name="name">
                </label>
                <button type="submit">
                    Save
                </button>
            </form>
        </Layout>
    }
}
