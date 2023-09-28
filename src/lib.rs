// Dioxus uses title case for component names
#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Otterdone(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, Otter! 🦦"
        }
    })
}
