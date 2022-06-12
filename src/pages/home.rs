use dioxus::{events::FormEvent, prelude::*};

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    let md = "Yoyo";

    cx.render(rsx! {
        h1 { "Home" }
        TextArea {
            value: md,
            oninput: |evt| {
                println!("{:?}", evt);
            }
        }
    })
}

#[derive(Props)]
struct TextAreaProps<'a> {
    value: &'a str,
    oninput: EventHandler<'a, FormEvent>,
}

#[allow(non_snake_case)]
fn TextArea<'a>(cx: Scope<'a, TextAreaProps<'a>>) -> Element {
    cx.render(rsx! {
        textarea {
            oninput: |evt| cx.props.oninput.call(evt),
            "{cx.props.value}"
        }
    })
}
