use dioxus::{events::FormEvent, prelude::*};

use super::MarkdownView;

#[allow(non_snake_case)]
pub fn CardEditor<'a>(cx: Scope<'a, CardEditorProps<'a>>) -> Element {
    let value = use_state(&cx, || cx.props.value.to_owned());

    cx.render(rsx! {
        textarea {
            rows: "10",
            cols: "80",
            oninput: |evt| {
                value.set(evt.value.clone());
            },
            onchange: |evt| cx.props.onchange.call(evt),
            "{cx.props.value}"
        }
        MarkdownView {
            text: "{value}",
        }
    })
}

#[derive(Props)]
pub struct CardEditorProps<'a> {
    value: &'a str,
    onchange: EventHandler<'a, FormEvent>,
}
