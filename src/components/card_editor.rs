use std::{
    ffi::OsStr,
    ops::Deref,
    path::{Path, PathBuf},
};

use dioxus::prelude::*;
use native_dialog::FileDialog;

use crate::services::config::Config;

use super::MarkdownView;

#[allow(non_snake_case)]
pub fn CardEditor<'a>(cx: Scope<'a, CardEditorProps<'a>>) -> Element {
    let cfg = &*cx.use_hook(|_| cx.consume_context::<Config>().unwrap());
    let content = use_state(&cx, || cx.props.initial_value.to_owned());

    // let path: PathBuf = "".into();
    // let name = path.file_name().unwrap();
    // let ext = path.extension().unwrap();
    // let file = std::fs::File::open(path).unwrap();

    cx.render(rsx! {
        img { src: "assets/img.png" }
        button {
            onclick: |_| {
                let path = FileDialog::new()
                // .set_location("~/Desktop")
                .add_filter("PNG Image", &["png"])
                .add_filter("JPEG Image", &["jpg", "jpeg"])
                .show_open_single_file()
                .unwrap();

                if let Some(path) = &path {
                    println!("PATH: {:?}", path);
                    // let file = std::fs::file::
                    // cx.props.onsave.call(content);
                    // content.set(String::new());
                    let bytes = std::fs::read(path).unwrap();
                    let digest = md5::compute(bytes);
                    let name = path.file_name().unwrap();
                    let ext = path.extension().unwrap();

                    let digest_name = format!("{:x}.{}", digest, ext.to_str().unwrap());
                    let target = &cfg.assets_dir.join(digest_name);

                    if !Path::exists(target) {
                        println!("COPY");
                        std::fs::copy(path, target).unwrap();
                    }


                    let mut md = content.current().deref().clone();
                    md.push_str("\n");
                    md.push_str(&format!("![]({})", target.display()));
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

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
