use std::ops::Deref;

use dioxus::prelude::*;
use native_dialog::FileDialog;

use super::MarkdownView;

#[allow(non_snake_case)]
pub fn CardEditor<'a>(cx: Scope<'a, CardEditorProps<'a>>) -> Element {
    let content = use_state(&cx, || cx.props.initial_value.to_owned());

    cx.render(rsx! {
        img { src: "assets/img.png" }
        button {
            onclick: |_| {
                let path = FileDialog::new()
                .set_location("~/Desktop")
                .add_filter("PNG Image", &["png"])
                .add_filter("JPEG Image", &["jpg", "jpeg"])
                .show_open_single_file()
                .unwrap();

                if let Some(path) = path {
                    println!("PATH: {:?}", path);
                    // cx.props.onsave.call(content);
                    // content.set(String::new());
                    let mut md = content.current().deref().clone();
                    md.push_str("\n");
                    md.push_str(&format!("![]({})", path.display()));
                    content.set(md);
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
