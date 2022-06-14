use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Home" }
    })
}
