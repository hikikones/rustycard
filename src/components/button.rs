use dioxus::{core::UiEvent, events::MouseData, prelude::*};

#[derive(Props)]
pub struct ButtonProps<'a> {
    name: &'a str,
    onclick: EventHandler<'a, UiEvent<MouseData>>,
}

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    cx.render(rsx! {
        button {
            class: "btn",
            onclick: |evt| {
                cx.props.onclick.call(evt);
            },
            "{cx.props.name}"
        }
    })
}
