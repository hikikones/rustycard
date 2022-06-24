use std::{cell::Cell};

use dioxus::prelude::*;

use crate::{components::MarkdownView, services::database::*};

#[allow(non_snake_case)]
pub fn Review(cx: Scope) -> Element {
    let db = &*cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let cards = use_ref(&cx, || db.get_due_cards());

    if cards.read().is_empty() {
        return cx.render(rsx! {
            h2 { "Done" }
        });
    }

    let index = &*cx.use_hook(|_| Cell::new(0));
    let show_count = &*cx.use_hook(|_| Cell::new(1));
    let show_amount = &*cx.use_hook(|_| Cell::new(split_count(&cards.read()[index.get()])));
    let show_content = use_state(&cx, || {
        split_content(&cards.read()[index.get()], show_count.get())
    });

    let buttons = match show_amount.get() == 1 || show_count.get() == show_amount.get() {
        true => rsx! {
            button {
                onclick: move |_| {
                    update_card_review(&cards.read()[index.get()], true, db);
                    cards.write_silent().swap_remove(index.get());
                    cards.with(|cards|{
                        if !cards.is_empty() {
                            index.set(index.get() % cards.len());
                            show_count.set(1);
                            show_amount.set(split_count(&cards[index.get()]));
                            show_content.set(split_content(&cards[index.get()], show_count.get()));
                        } else {
                            cx.needs_update();
                        }
                    });
                },
                "Yes"
            }
            button {
                onclick: move |_| {
                    update_card_review(&cards.read()[index.get()], false, db);
                    cards.write_silent().swap_remove(index.get());
                    cards.with(|cards|{
                        if !cards.is_empty() {
                            index.set(index.get() % cards.len());
                            show_count.set(1);
                            show_amount.set(split_count(&cards[index.get()]));
                            show_content.set(split_content(&cards[index.get()], show_count.get()));
                        } else {
                            cx.needs_update();
                        }
                    });
                },
                "No"
            }
        },
        false => rsx! {
            button {
                onclick: move |_| {
                    show_count.set(show_count.get() + 1);
                    show_content.set(split_content(&cards.read()[index.get()], show_count.get()));
                },
                "Show"
            }
        },
    };

    cx.render(rsx! {
        h1 { "Review" }
        MarkdownView {
            text: show_content
        }
        buttons
        button {
            onclick: move |_| {
                index.set((index.get() + 1) % cards.read().len());
                show_count.set(1);
                show_amount.set(split_count(&cards.read()[index.get()]));
                show_content.set(split_content(&cards.read()[index.get()], show_count.get()));
            },
            "Skip"
        }
    })
}

// FIXME: Parse <hr> tags properly with an html parser or something.
fn split_content(card: &Card, count: usize) -> String {
    let mut split = split_iter(card);
    let mut s = split.next().unwrap().to_string();
    for _ in 1..count {
        s.push_str("---");
        s.push_str(split.next().unwrap());
    }
    s
}

fn split_count(card: &Card) -> usize {
    split_iter(card).count()
}

fn split_iter(card: &Card) -> std::str::Split<&str> {
    card.content.split("---")
}

fn update_card_review(card: &Card, success: bool, db: &Database) {
    let due_days = if success {
        (card.due_days * 2).max(1)
    } else {
        card.due_days / 2
    };
    let due_date = chrono::Utc::now() + chrono::Duration::days(due_days as i64);
    db.update_card_review(card.id, due_date.date().naive_utc(), due_days);
}
