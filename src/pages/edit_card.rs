use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn EditCard(cx: Scope) -> Element {
    let id = use_route(&cx)
        .segment("id")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(0);

    cx.render(rsx! {
        div {
            h1 { "Edit card" }
            p { "Id: {id}" }
        }
    })
}
