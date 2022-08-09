use dioxus::prelude::*;

use crate::{components::CardEditor, services::database::use_database};

#[allow(non_snake_case)]
pub fn AddCard(cx: Scope) -> Element {
    let db = use_database(&cx);

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
