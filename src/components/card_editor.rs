use dioxus::{events::FormEvent, prelude::*};

use super::MarkdownView;

#[allow(non_snake_case)]
pub fn CardEditor<'a>(cx: Scope<'a, CardEditorProps<'a>>) -> Element {
    let content = use_state(&cx, || cx.props.value.clone());

    cx.render(rsx! {
        textarea {
            rows: "10",
            cols: "80",
            value: "{content}",
            oninput: |evt| {
                content.set(evt.value.clone());
                // cx.props.oninput.call(evt);
            },
        }
        MarkdownView {
            text: "{content}",
        }
        button {
            onclick: move |_| {
                (cx.props.save)(content);
            },
            "Save"
        }
    })
}

#[derive(Props)]
pub struct CardEditorProps<'a> {
    // oninput: EventHandler<'a, FormEvent>,
    #[props(default)]
    value: String,
    save: &'a dyn Fn(&str),
}
