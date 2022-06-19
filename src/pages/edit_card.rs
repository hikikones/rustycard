use dioxus::{events::FormEvent, prelude::*};

use crate::{components::CardEditor, services::database::*};

// TODO: Go back when done.

#[allow(non_snake_case)]
pub fn EditCard(cx: Scope) -> Element {
    let id = use_route(&cx)
        .segment("id")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    assert!(id != 0);

    let db = &*cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let markdown = use_state(&cx, || db.get_card(id).content);
    let done = use_state(&cx, || false);

    if *done.current() {
        return cx.render(rsx! {
            h1 { "Done" }
        });
    }

    cx.render(rsx! {
        h1 { "Edit card" }
        p { "Id: {id}" }
        CardEditor {
            value: db.get_card(id).content,
            save: &|text| {
                println!("edit save: {text}");
            },
            // oninput: |evt: FormEvent| {
            //     println!("{:?}", evt);
            //     markdown.set(evt.value.clone());
            // }
        }
        // button {
        //     onclick: move |_| {
        //         if !markdown.is_empty() {
        //             println!("Edit card!");
        //             db.update_card_content(id, markdown);
        //             done.set(true);
        //         }
        //     },
        //     "Save"
        // }
    })
}
