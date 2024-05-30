#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

pub fn Field(name: &str, typ: &str, label: Elements) -> Component {
    html! {
        <div class="mb-12">
            <label class="block mb-12" for=name>
                {label}
            </label>
            <input class="block w-full rounded text-black" type=typ name=name>
        </div>
    }
}

pub fn Button(children: Elements) -> Component {
    html! {
        <button class="rounded bg-gray-700 px-12">
            {children}
        </button>
    }
}

pub fn CancelButton(children: Elements) -> Component {
    html! {
        <button class="rounded bg-gray-700 px-12" value="cancel" formmethod="dialog">
            {children}
        </button>
    }
}

pub fn ButtonLink(href: &str, children: Elements) -> Component {
    html! {
        <a href=href class="rounded bg-gray-700 px-12">
            {children}
        </a>
    }
}
