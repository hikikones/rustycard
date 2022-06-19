use dioxus::prelude::*;

mod components;
mod config;
mod database;
mod markdown;
mod pages;

fn main() {
    // TODO: Create config and database here.
    // let cfg = config::Config::new();
    dioxus::desktop::launch(app);
    // dioxus::desktop::launch_with_props(app, (), |c| c.with_resource_directory());
}

fn app(cx: Scope) -> Element {
    cx.use_hook(|_| {
        let cfg = config::Config::new();
        let db = database::Database::new(&cfg.db_file);
        cx.provide_context(cfg);
        cx.provide_context(db);
    });
    // let cfg: &config::Config = cx.use_hook(|_| cx.consume_context::<config::Config>().unwrap());
    // use_context_provider(&cx, || config::Config::new());
    // use_context_provider(&cx, || {
    //     let db_file = {
    //         let cfg = use_context::<config::Config>(&cx).unwrap();
    //         let x = cfg.read().db_file.clone();
    //         x
    //     };
    //     database::Database::new(&db_file);
    // });
    // use_context_provider(&cx, || database::Database::new("db.sqlite3"));

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
