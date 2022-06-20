use dioxus::prelude::*;

use super::MarkdownView;

#[allow(non_snake_case)]
pub fn CardEditor<'a>(cx: Scope<'a, CardEditorProps<'a>>) -> Element {
    let content = use_state(&cx, || cx.props.initial_value.to_owned());

    cx.render(rsx! {
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
