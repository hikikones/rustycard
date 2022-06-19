use dioxus::prelude::*;

mod components;
mod config;
mod database;
mod markdown;
mod pages;

fn main() {
    // TODO: Create config and database here.
    let cfg = config::Config::new();
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    use_context_provider(&cx, || database::Database::new("db.sqlite3"));

    cx.render(rsx! {
        Router {
            h1 { "Navigation" }
            ul {
                Link { to: "/review", li { "Review"  }}
                Link { to: "/cards", li { "Cards"  }}
                Link { to: "/add_card", li { "Add card"  }}
                Link { to: "/edit_card/1", li { "Edit card"  }}
            }
            Route { to: "/review", pages::Review {} }
            Route { to: "/cards", pages::Cards {} }
            Route { to: "/add_card", pages::AddCard {} }
            Route { to: "/edit_card/:id", pages::EditCard {} }
            Redirect { from: "", to: "/review" }
        }
    })
}
