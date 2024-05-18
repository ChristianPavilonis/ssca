#![allow(non_snake_case)]

use shtml::{html as view, Component, Elements, Render};

pub fn Layout(elements: Elements) -> Component {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                <meta name="viewport" content="width=device-width,initial-scale=1.0" />
                <link rel="stylesheet" href="/app.css">
                <script src="https://unpkg.com/htmx.org@1.9.12"></script>
                <script src="https://unpkg.com/htmx.org@1.9.12/dist/ext/ws.js"></script>
                <title>SHAT STACK</title>
            </head>
            <body class="bg-gray-800 text-white min-h-[100vh]">
                <main class="mx-auto max-w-[600px] pt-40 ">
                    {elements}
                </main>
            </body>
        </html>
    }
}
