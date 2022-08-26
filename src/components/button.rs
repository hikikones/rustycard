use dioxus::{core::UiEvent, events::MouseData, prelude::*};

#[derive(Props)]
pub struct ButtonProps<'a> {
    name: &'a str,
    onclick: EventHandler<'a, UiEvent<MouseData>>,

    #[props(default)]
    disabled: bool,
}

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    cx.render(rsx! {
        button {
            class: "btn",
            disabled: "{cx.props.disabled}",
            onclick: |evt| {
                cx.props.onclick.call(evt);
            },
            "{cx.props.name}"
        }
    })
}
