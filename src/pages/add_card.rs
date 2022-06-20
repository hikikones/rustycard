use dioxus::{events::FormEvent, prelude::*};

use crate::{
    components::{CardEditor, MarkdownView},
    services::database::Database,
};

#[allow(non_snake_case)]
pub fn AddCard(cx: Scope) -> Element {
    let db = &*cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
    let markdown = use_state(&cx, || String::from("# Yoyo"));

    // let save = &*cx.use_hook(|_| {
    //     move |text: &str| {
    //         if !text.is_empty() {
    //             db.create_card(text);
    //         }
    //     }
    // });

    cx.render(rsx! {
        h1 { "Add card" }
        CardEditor {
            onsave: |text: &str| {
                println!("SAVE: {}", text);
                db.create_card(text);
            },
            // save_callback: &move |text| {
            //     println!("SAVE!: {text}");
            //     if text.is_empty() {
            //         // let c = db.get_card(1);
            //         // let s = String::from(text);
            //         db.create_card(text);
            //     }
            // },
            // oninput: |evt: FormEvent| {
            //     println!("{:?}", evt);
            //     markdown.set(evt.value.clone());
            // }
        }
        // button {
        //     onclick: move |_| {
        //         if !markdown.is_empty() {
        //             println!("Save card!");
        //             db.create_card(markdown);
        //             markdown.set(String::new());
        //         }
        //     },
        //     "Save"
        // }
    })
}
