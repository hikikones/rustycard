use dioxus::prelude::*;

use crate::{components::CardEditor, services::database::*};

// TODO: Go back when done.

#[allow(non_snake_case)]
pub fn EditCard(cx: Scope) -> Element {
    let id = use_route(&cx)
        .segment("id")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    assert!(id != 0);

    let db = &*cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let done = use_state(&cx, || false);

    if *done.current() {
        return cx.render(rsx! {
            h1 { "Done" }
        });
    }

    cx.render(rsx! {
        h1 { "Edit card" }
        p { "Id: {id}" }
        CardEditor {
            initial_value: db.get_card(id).content,
            onsave: move |content: &str| {
                if !content.is_empty() {
                    db.update_card_content(id, content);
                    done.set(true);
                }
            },
        }
    })
}
