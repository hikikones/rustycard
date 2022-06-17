use dioxus::prelude::*;

use crate::{components::MarkdownView, database::Database};

#[allow(non_snake_case)]
pub fn Cards(cx: Scope) -> Element {
    let db = use_context::<Database>(&cx).unwrap();

    cx.render(rsx! {
        h1 { "Cards" }
        hr {}
        db.read().get_cards().iter().map(|c| rsx! {
            MarkdownView {
                key: "{c.id}",
                text: "{c.content}",
            }
        })

        h1 { "Tags" }
        hr {}
        ul {
            db.read().get_tags().iter().map(|t| rsx! {
                li {
                    key: "{t.id}",
                    "{t.name}",
                }
            })
        }

        h1 { "Cards with tag1 & tag2" }
        hr {}
        db.read().get_cards_by_tags(&[1,2]).iter().map(|c| rsx! {
            MarkdownView {
                key: "{c.id}",
                text: "{c.content}",
            }
        })
    })
}
