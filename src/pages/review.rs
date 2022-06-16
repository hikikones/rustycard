use std::collections::VecDeque;

use dioxus::prelude::*;

use crate::{components::MarkdownView, database::*};

#[allow(non_snake_case)]
pub fn Review(cx: Scope) -> Element {
    let db = cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let cards = cx.use_hook(|_| get_due_cards(db));
    let current_card = use_state(&cx, || cards.pop_front());

    let review = match &**current_card {
        Some(card) => rsx! {
            MarkdownView {
                text: &card.content
            }
            button {
                onclick: |_| {
                    current_card.set(cards.pop_front());
                },
                "Next"
            }
        },
        None => rsx! {
            h2 { "Done" }
        },
    };

    cx.render(review)
}

fn get_due_cards(db: &Database) -> VecDeque<Card> {
    println!("GET_DUE_CARDS");
    VecDeque::from_iter(db.get_cards())
}
