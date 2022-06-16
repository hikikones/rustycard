use std::collections::VecDeque;

use dioxus::prelude::*;

use crate::{components::MarkdownView, database::*};

#[allow(non_snake_case)]
pub fn Review(cx: Scope) -> Element {
    let db = cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let cards = cx.use_hook(|_| get_due_cards(db));
    let card = use_state(&cx, || cards.pop_front());

    let render = match &**card {
        Some(card) => rsx!(MarkdownView {
            text: &card.content
        }),
        None => rsx!(h2 { "Done" }),
    };

    cx.render(rsx! {
        h1 { "Review" }
        render
        button {
            onclick: |_| {
                card.set(cards.pop_front());
            },
            "Next"
        }
    })
}

fn get_due_cards(db: &Database) -> VecDeque<Card> {
    println!("GET_DUE_CARDS");
    VecDeque::from_iter(db.get_cards())
}
