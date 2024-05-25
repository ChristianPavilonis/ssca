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
