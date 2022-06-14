use dioxus::{events::FormEvent, prelude::*};

use crate::{components::CardEditor, database::*};

// TODO: Go back when done.

#[allow(non_snake_case)]
pub fn EditCard(cx: Scope) -> Element {
    let id = use_route(&cx)
        .segment("id")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    assert!(id != 0);

    let db = cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let markdown = use_state(&cx, || db.get_card(id).content);

    cx.render(rsx! {
        h1 { "Edit card" }
        p { "Id: {id}" }
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
                    println!("Edit card!");
                    db.update_card_content(id, markdown);
                    markdown.set(String::new());
                }
            },
            "Save"
        }
    })
}
