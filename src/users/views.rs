#![allow(non_snake_case)]

use crate::components::Field;
use crate::layouts::Layout;
use shtml::{html as view, Component, Render};

fn Register() -> Component {
    view! {
        <form action="/register" method="post">
            <Field name="name" typ="text">
                Name
            </Field>

            <Field name="password" typ="password">
                Password
            </Field>
        </form>
    }
}
