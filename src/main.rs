use std::rc::Rc;

use dioxus::{desktop::use_window, prelude::*};

use services::{
    config::Config,
    database::{use_database, Database},
};

mod components;
mod pages;
mod services;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.use_hook(|_| {
        let cfg = Config::new();
        let db = Rc::new(Database::new(&cfg));
        cx.provide_context(cfg);
        cx.provide_context(db);
    });

    let cfg = &*cx.use_hook(|_| cx.consume_context::<Config>().unwrap());
    let db = use_database(&cx);
    let window = use_window(&cx);

    cx.render(rsx! {
        Router {
            h1 { "Navigation" }
            ul {
                Link { to: "/review", li { "Review"  }}
                Link { to: "/cards", li { "Cards"  }}
                Link { to: "/add_card", li { "Add card"  }}
                Link { to: "/edit_card/1", li { "Edit card"  }}
                Link { to: "/settings", li { "Settings"  }}
            }
            button {
                onclick: move |_| {
                    cfg.save();
                    db.save(cfg);
                    window.close();
                },
                "Quit"
            }
            Route { to: "/review", pages::Review {} }
            Route { to: "/cards", pages::Cards {} }
            Route { to: "/add_card", pages::AddCard {} }
            Route { to: "/edit_card/:id", pages::EditCard {} }
            Route { to: "/settings", pages::Settings {} }
            Redirect { from: "", to: "/review" }
        }
    })
}
