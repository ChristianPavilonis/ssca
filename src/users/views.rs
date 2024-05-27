#![allow(non_snake_case)]

use crate::components::Button;
use crate::components::Field;
use crate::layouts::Layout;
use shtml::{html, Component, Render};

pub fn Register() -> Component {
    html! {
        <Layout>
            <form action="/register" method="post">
                <Field name="name" typ="text">
                    Name
                </Field>

                <Field name="password" typ="password">
                    Password
                </Field>
                <Button>
                    Register
                </Button>
            </form>
        </Layout>
    }
}

// maybe instead of a full page it could be a html dialog element
pub fn Login() -> Component {
    html! {
        <Layout>
            <form action="/login" method="post">
                <Field name="name" typ="text">
                    Name
                </Field>

                <Field name="password" typ="password">
                    Password
                </Field>
                <Button>
                    Login
                </Button>
            </form>
        </Layout>
    }
}
