use dioxus::{events::FormEvent, prelude::*};

use crate::{
    components::{CardEditor, MarkdownView},
    database::*,
};

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    let db = cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let markdown = use_state(&cx, || String::from("# Yoyo"));

    cx.render(rsx! {
        h1 { "Home" }

        h2 { "Create card" }
        hr {}
        CardEditor {
            value: markdown,
            oninput: |evt: FormEvent| {
                println!("{:?}", evt);
                markdown.set(evt.value.clone());
            }
        }
        button {
            onclick: |_| {
                if !markdown.is_empty() {
                    println!("Save card!");
                    db.create_card(markdown);
                    markdown.set(String::new());
                }
            },
            "Save"
        }

        h2 { "Cards" }
        hr {}
        db.get_cards().iter().map(|c| rsx! {
            MarkdownView {
                key: "{c.id}",
                text: "{c.content}",
            }
        })
    })
}
