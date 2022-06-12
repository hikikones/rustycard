use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Home" }
        TextArea { text: "Yoyo".into() }
    })
}

#[derive(Props, PartialEq)]
struct AppProps {
    text: String,
}

#[allow(non_snake_case)]
fn TextArea(cx: Scope<AppProps>) -> Element {
    cx.render(rsx! {
        textarea {
            oninput: |evt| {
                println!("{:?}", evt);
            },
            "{cx.props.text}"
        }
    })
}
