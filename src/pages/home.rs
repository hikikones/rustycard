use dioxus::{events::FormEvent, prelude::*};

use crate::components::CardEditor;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    let markdown = use_state(&cx, || String::from("# Yoyo"));

    cx.render(rsx! {
        h1 { "Home" }
        CardEditor {
            value: markdown,
            onchange: |evt: FormEvent| {
                println!("{:?}", evt);
                markdown.set(evt.value.clone());
            }
        }
    })
}
