use dioxus::prelude::*;

use crate::services::config::Config;

#[allow(non_snake_case)]
pub fn Settings(cx: Scope) -> Element {
    let cfg = &*cx.use_hook(|_| cx.consume_context::<Config>().unwrap());
    let db_location = use_state(&cx, || cfg.get_db_file_path().display().to_string());

    cx.render(rsx! {
        h1 { "Settings" }

        h2 { "Database" }
        h3 { "Location" }
        span { "{db_location}" }
        br {}
        button {
            onclick: move |_| {
                //todo
            },
            "Change"
        }
    })
}
