use dioxus::prelude::*;

use crate::{components::MarkdownView, database::*};

fn split_text(text: &str, count: usize) -> String {
    let mut split = text.split("---");
    let mut s = String::new();
    for _ in 0..count {
        println!("PUSH");
        s.push_str(split.next().unwrap());
    }
    println!("{s}");
    s
}

#[allow(non_snake_case)]
pub fn Review(cx: Scope) -> Element {
    let db = cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let cards = cx.use_hook(|_| get_due_cards(db));

    if cards.is_empty() {
        return cx.render(rsx! {
            h2 { "Done" }
        });
    }

    let index = cx.use_hook(|_| 0usize);
    let show_count = cx.use_hook(|_| 1usize);
    let show_amount = cx.use_hook(|_| cards[*index].content.split("---").count());
    let content = use_state(&cx, || split_text(&cards[*index].content, *show_count));

    cx.render(rsx! {
        h1 { "Review" }
        MarkdownView {
            text: content
        }
        button {
            onclick: move |_| {
                *show_count += 1;
                if *show_count > *show_amount {
                    *show_count = 1;
                }
                content.set(split_text(&cards[*index].content, *show_count));
            },
            "Show"
        }
    })
}

fn get_due_cards(db: &Database) -> Vec<Card> {
    println!("GET_DUE_CARDS");
    db.get_cards()
}
