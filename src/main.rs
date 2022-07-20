use dioxus::prelude::*;

mod components;
mod pages;
mod services;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.use_hook(|_| {
        let cfg = services::config::Config::new();
        let db = services::database::Database::new(cfg.get_db_file_path());
        cx.provide_context(cfg);
        cx.provide_context(db);
    });

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
