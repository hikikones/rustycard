use dioxus::prelude::*;
use native_dialog::FileDialog;

use crate::services::config::use_config;

#[allow(non_snake_case)]
pub fn Settings(cx: Scope) -> Element {
    let cfg = use_config(&cx);
    let location = use_state(&cx, || {
        cfg.borrow()
            .get_location()
            .map_or("None".to_string(), |loc| loc.display().to_string())
    });

    cx.render(rsx! {
        h1 { "Settings" }

        h2 { "Database" }
        h3 { "Location" }
        span { "{location}" }
        br {}
        button {
            onclick: move |_| {
                let path = FileDialog::new()
                    .add_filter("Rustyzip", &["rustyzip"])
                    .show_save_single_file()
                    .unwrap();

                if let Some(path) = &path {
                    cfg.borrow_mut().set_location(path);
                    location.set(path.display().to_string());
                }
            },
            "Change"
        }
    })
}
