use std::collections::HashSet;

use dioxus::prelude::*;

use crate::{components::MarkdownView, services::database::*};

#[derive(PartialEq, Props)]
struct TagEl {
    id: usize,
    name: String,
    selected: bool,
}

impl From<&Tag> for TagEl {
    fn from(tag: &Tag) -> Self {
        Self {
            id: tag.id,
            name: tag.name.clone(),
            selected: false,
        }
    }
}

#[allow(non_snake_case)]
pub fn Cards(cx: Scope) -> Element {
    let db = &*cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let cards = use_state(&cx, || db.get_cards());
    let tags = use_state(&cx, || db.get_tags());
    let selected_tags = use_state(&cx, || HashSet::<usize>::new());
    let show_tagless = use_state(&cx, || false);

    cx.render(rsx! {
        h1 { "All Cards" }

        h2 { "Tags" }
        button {
            onclick: |_| {
                show_tagless.set(false);
                selected_tags.make_mut().clear();
                cards.set(db.get_cards());
            },
            "Reset"
        }
        br {}
        span {
            color: format_args!("{}", if **show_tagless {"blue"} else {"black"}),
            onclick: |_| {
                // *show_tagless.make_mut() = !*show_tagless.current();
                let mut show = false;
                show_tagless.with_mut(|s| {
                    *s = !*s;
                    // println!("SHOW: {}", *show_tagless);
                    show = *s;
                });
                // show_tagless.set(!*show_tagless.current());
                println!("SHOW: {}", show);
                if show {
                    cards.set(db.get_cards_without_tags());
               } else {
                    cards.set(db.get_cards_by_tags(&selected_tags.current().iter().copied().collect::<Vec<_>>()));
               }
            },
            "tagless",
        }
        br {}
        tags.iter().map(|t| rsx! {
            span {
                key: "{t.id}",
                // color: format_args!("{}", if t.selected {"blue"} else {"black"}),
                color: format_args!("{}", if !**show_tagless && selected_tags.contains(&t.id) {"blue"} else {"black"}),
                onclick: |_| {
                    // println!("TAG ID: {}", t.id);
                    show_tagless.set(false);
                    selected_tags.with_mut(|tags| {
                        if tags.contains(&t.id) {
                            tags.remove(&t.id);
                        } else {
                            tags.insert(t.id);
                        }
                    });
                    cards.set(db.get_cards_by_tags(&selected_tags.current().iter().copied().collect::<Vec<_>>()));
                },
                "{t.name} \t",
            }
        })

        h2 { "Cards" }
        cards.iter().map(|c| rsx! {
            MarkdownView {
                key: "{c.id}",
                text: "{c.content}",
            }
        })

        // hr {}
        // db.get_cards().iter().map(|c| rsx! {
        //     MarkdownView {
        //         key: "{c.id}",
        //         text: "{c.content}",
        //     }
        // })

        // h1 { "Tags" }
        // hr {}
        // ul {
        //     db.get_tags().iter().map(|t| rsx! {
        //         li {
        //             key: "{t.id}",
        //             "{t.name}",
        //         }
        //     })
        // }

        // h1 { "Cards with tag1 & tag2" }
        // hr {}
        // db.get_cards_by_tags(&[1,2]).iter().map(|c| rsx! {
        //     MarkdownView {
        //         key: "{c.id}",
        //         text: "{c.content}",
        //     }
        // })

        // h1 { "Tagless cards" }
        // hr {}
        // db.get_cards_without_tags().iter().map(|c| rsx! {
        //     MarkdownView {
        //         key: "{c.id}",
        //         text: "{c.content}",
        //     }
        // })
    })
}
