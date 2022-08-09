use std::cell::Cell;

use dioxus::prelude::*;

use crate::{components::MarkdownView, services::database::*};

#[allow(non_snake_case)]
pub fn Review(cx: Scope) -> Element {
    let db = use_database(&cx);
    let cards = use_ref(&cx, || db.borrow().get_due_cards());

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

    let is_card_fully_shown = show_count.get() == show_amount.get();
    let review_buttons = match is_card_fully_shown {
        true => rsx! {
            button {
                onclick: move |_| {
                    update_card_review(&cards.read()[index.get()], true, &*db.borrow());
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
                    update_card_review(&cards.read()[index.get()], false, &*db.borrow());
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
        review_buttons
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
    let mut review = card.review.clone();
    review.recall_attempts += 1;
    if success {
        review.successful_recalls += 1;
        review.due_days = (review.due_days * 2).max(1);
    } else {
        review.due_days = review.due_days / 2;
    };
    review.due_date = (chrono::Utc::now() + chrono::Duration::days(review.due_days as i64))
        .date()
        .naive_utc();
    db.update_card_review(card.id, review);
}
