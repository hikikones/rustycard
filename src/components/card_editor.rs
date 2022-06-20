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
        img { src: "assets/img.png" }
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
                    let digest_name = format!("{:x}.{}", digest, path.extension().unwrap().to_str().unwrap());
                    let target = &cfg.assets_dir.join(digest_name);

                    if !Path::exists(target) {
                        std::fs::copy(path, target).unwrap();
                    }

                    content.with_mut(|c|{
                        c.push_str("\n");
                        c.push_str(&format!("![]({})", target.display()));
                    });
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
