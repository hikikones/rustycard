use std::path::Path;

use dioxus::prelude::*;
use native_dialog::FileDialog;

use crate::services::config::Config;

use super::MarkdownView;

#[allow(non_snake_case)]
pub fn CardEditor<'a>(cx: Scope<'a, CardEditorProps<'a>>) -> Element {
    let cfg = &*cx.use_hook(|_| cx.consume_context::<Config>().unwrap());
    let content = use_state(&cx, || cx.props.initial_value.to_owned());

    cx.render(rsx! {
        button {
            onclick: |_| {
                let path = FileDialog::new()
                .add_filter("PNG Image", &["png"])
                .add_filter("JPEG Image", &["jpg", "jpeg"])
                .show_open_single_file()
                .unwrap();

                if let Some(path) = &path {
                    let bytes = std::fs::read(path).unwrap();
                    let digest = md5::compute(bytes);
                    let ext = path.extension().unwrap().to_str().unwrap();
                    let filename = format!("{:x}.{}", digest, ext);

                    let target_path = &cfg.get_assets_dir().join(&filename);
                    if !Path::exists(target_path) {
                        std::fs::copy(path, target_path).unwrap();
                    }

                    let img = cfg.get_assets_dir_name().to_owned() + "/" + &filename;
                    content.make_mut().push_str(&format!("\n![]({img})"));
                }
            },
            "Image"
        }
        textarea {
            rows: "10",
            cols: "80",
            value: "{content}",
            oninput: |evt| {
                content.set(evt.value.clone());
            },
        }
        MarkdownView {
            text: "{content}",
        }
        button {
            onclick: |_| {
                cx.props.onsave.call(content);
                content.set(String::new());
            },
            "Save"
        }
    })
}

#[derive(Props)]
pub struct CardEditorProps<'a> {
    #[props(default)]
    initial_value: String,
    onsave: EventHandler<'a, &'a str>,
}
