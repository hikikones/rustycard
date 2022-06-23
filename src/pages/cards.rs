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
    // let tags = use_state(&cx, || {
    //     db.get_tags()
    //         .iter()
    //         .map(|t| TagEl::from(t))
    //         .collect::<Vec<_>>()
    // });

    // let selected_cards = use_state(&cx, || HashSet::<usize>::new());
    let selected_tags = use_state(&cx, || HashSet::<usize>::new());

    // let a = selected_tags.current().iter().cloned().collect::<Vec<_>>();

    cx.render(rsx! {
        h1 { "All Cards" }

        h2 { "Tags" }
        tags.iter().map(|t| rsx! {
            span {
                key: "{t.id}",
                // color: format_args!("{}", if t.selected {"blue"} else {"black"}),
                color: format_args!("{}", if selected_tags.contains(&t.id) {"blue"} else {"black"}),
                onclick: |_| {
                    // println!("TAG ID: {}", t.id);
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
