use dioxus::prelude::*;

use crate::{components::CardEditor, services::database::Database};

#[allow(non_snake_case)]
pub fn AddCard(cx: Scope) -> Element {
    let db = &*cx.use_hook(|_| cx.consume_context::<Database>().unwrap());

    cx.render(rsx! {
        h1 { "Add card" }
        CardEditor {
            onsave: |content: &str| {
                if !content.is_empty() {
                    db.create_card(content);
                }
            },
        }
    })
}
