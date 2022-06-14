use dioxus::{events::FormEvent, prelude::*};

use super::MarkdownView;

#[allow(non_snake_case)]
pub fn CardEditor<'a>(cx: Scope<'a, CardEditorProps<'a>>) -> Element {
    let value = use_state(&cx, || cx.props.value.to_owned());

    cx.render(rsx! {
        textarea {
            rows: "10",
            cols: "80",
            value: "{cx.props.value}",
            oninput: |evt| cx.props.oninput.call(evt),
        }
        MarkdownView {
            text: "{value}",
        }
    })
}

#[derive(Props)]
pub struct CardEditorProps<'a> {
    value: &'a str,
    oninput: EventHandler<'a, FormEvent>,
}
