use dioxus::{events::FormEvent, prelude::*};

use super::MarkdownView;

#[allow(non_snake_case)]
pub fn CardEditor<'a>(cx: Scope<'a, CardEditorProps<'a>>) -> Element {
    cx.render(rsx! {
        textarea {
            rows: "10",
            cols: "80",
            value: "{cx.props.value}",
            oninput: |evt| cx.props.oninput.call(evt),
        }
        MarkdownView {
            text: "{cx.props.value}",
        }
    })
}

#[derive(Props)]
pub struct CardEditorProps<'a> {
    value: &'a str,
    oninput: EventHandler<'a, FormEvent>,
}
