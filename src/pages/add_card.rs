use dioxus::{events::FormEvent, prelude::*};

use crate::{components::CardEditor, services::database::Database};

#[allow(non_snake_case)]
pub fn AddCard(cx: Scope) -> Element {
    let db = &*cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let markdown = use_state(&cx, || String::from("# Yoyo"));

    cx.render(rsx! {
        h1 { "Add card" }
        CardEditor {
            value: markdown,
            oninput: |evt: FormEvent| {
                println!("{:?}", evt);
                markdown.set(evt.value.clone());
            }
        }
        button {
            onclick: move |_| {
                if !markdown.is_empty() {
                    println!("Save card!");
                    db.create_card(markdown);
                    markdown.set(String::new());
                }
            },
            "Save"
        }
    })
}
