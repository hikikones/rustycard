use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            h1 { "Navigation" }
            ul {
                Link { to: "/", li { "Home"  }}
                Link { to: "/cards", li { "Cards"  }}
                Link { to: "/add_card", li { "Add card"  }}
                Link { to: "/edit_card/1", li { "Edit card"  }}
            }
            Route { to: "/", Home {} }
            Route { to: "/cards", Cards {} }
            Route { to: "/add_card", AddCard {} }
            Route { to: "/edit_card/:id", EditCard {} }
            Redirect { from: "", to: "/" }
        }
    })
}

#[allow(non_snake_case)]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Home" }
    })
}

#[allow(non_snake_case)]
fn Cards(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Cards" }
    })
}

#[allow(non_snake_case)]
fn AddCard(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Add card" }
    })
}

#[allow(non_snake_case)]
fn EditCard(cx: Scope) -> Element {
    let id = use_route(&cx)
        .segment("id")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(0);

    cx.render(rsx! {
        div {
            h1 { "Edit card" }
            p { "Card to edit: {id}" }
        }
    })
}
