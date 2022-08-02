use std::{cell::RefCell, fs::File, path::Path};

use dioxus::{desktop::use_window, prelude::*};

use services::{config::Config, database::Database, ServiceLocator};

mod components;
mod pages;
mod services;

thread_local! {
    static SERVICES: RefCell<ServiceLocator> = RefCell::new(ServiceLocator::new());
}

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.use_hook(|_| {
        let cfg = Config::new();

        let app_db_file = cfg.get_app_db_file();
        if let Some(custom_db_file) = cfg.get_custom_db_file() {
            if sync_file(&custom_db_file, &app_db_file) {
                //todo
            }
        }

        let db = Database::new(&app_db_file);
        cx.provide_context(cfg);
        cx.provide_context(db);
    });

    let cfg = &*cx.use_hook(|_| cx.consume_context::<Config>().unwrap());
    let db = &*cx.use_hook(|_| cx.consume_context::<Database>().unwrap());
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
                    cfg.write();
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

fn sync_file(file: &Path, target: &Path) -> bool {
    let f1 = File::open(file).unwrap();
    let f2 = File::open(target).unwrap();

    let m1 = f1.metadata().unwrap();
    let m2 = f2.metadata().unwrap();

    if m1.len() == m2.len() {
        return false;
    }

    if m1.modified().unwrap() <= m2.modified().unwrap() {
        return false;
    }

    // Newer file. Copy over.
    std::fs::copy(file, target).unwrap();

    true
}
