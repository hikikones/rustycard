use dioxus::prelude::*;

use crate::{components::MarkdownView, database::*};

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

    let buttons = match *show_amount == 1 || *show_count == *show_amount {
        true => rsx! {
            button {
                onclick: move |_| {
                    cards.swap_remove(*index);
                    if !cards.is_empty() {
                        *show_count = 1;
                        *show_amount = cards[*index].content.split("---").count();
                        content.set(split_text(&cards[*index].content, *show_count));
                    } else {
                        cx.needs_update();
                    }
                },
                "Next"
            }
        },
        false => rsx! {
            button {
                onclick: |_| {
                    *show_count += 1;
                    content.set(split_text(&cards[*index].content, *show_count));
                },
                "Show"
            }
        },
    };

    cx.render(rsx! {
        h1 { "Review" }
        MarkdownView {
            text: content
        }
        buttons
    })
}

fn get_due_cards(db: &Database) -> Vec<Card> {
    db.get_cards()
}

fn split_text(text: &str, count: usize) -> String {
    let mut split = text.split("---");
    let mut s = split.next().unwrap().to_string();
    for _ in 1..count {
        s.push_str("---");
        s.push_str(split.next().unwrap());
    }
    s
}
