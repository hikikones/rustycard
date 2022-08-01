use dioxus::prelude::*;
use native_dialog::FileDialog;

use crate::services::config::Config;

#[allow(non_snake_case)]
pub fn Settings(cx: Scope) -> Element {
    let cfg = &*cx.use_hook(|_| cx.consume_context::<Config>().unwrap());
    let db_path = use_state(&cx, || cfg.get_db_file_path().display().to_string());
    let assets_path = use_state(&cx, || cfg.get_assets_dir_path().display().to_string());

    cx.render(rsx! {
        h1 { "Settings" }

        h2 { "Database" }
        h3 { "Location" }
        span { "{db_path}" }
        br {}
        button {
            onclick: move |_| {
                let path = FileDialog::new()
                    .add_filter("Lazycard database", &["db"])
                    .show_save_single_file()
                    .unwrap();

                if let Some(path) = &path {
                    cfg.set_custom_db_file_path(path);
                    db_path.set(path.display().to_string());
                }
            },
            "Change"
        }
        h3 { "Assets" }
        span { "{assets_path}" }
        br {}
        button {
            onclick: move |_| {
                let path = FileDialog::new()
                    .show_open_single_dir()
                    .unwrap();

                if let Some(path) = &path {
                    cfg.set_custom_assets_dir_path(path);
                    assets_path.set(path.display().to_string());
                }
            },
            "Change"
        }
    })
}
