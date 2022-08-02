use std::{cell::RefCell, collections::HashSet, fs::File, path::Path};

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
                if let Some(custom_assets_dir) = cfg.get_custom_assets_dir() {
                    sync_dir(&custom_assets_dir, &cfg.get_app_assets_dir());
                }
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

                    println!("\n\nIS_DIRTY: {}\n\n", db.is_dirty());

                    if db.is_dirty() {
                        if let Some(custom_db_file) = cfg.get_custom_db_file() {
                            std::fs::copy(&cfg.get_app_db_file(), &custom_db_file).unwrap();
                            if let Some(custom_assets_dir) = cfg.get_custom_assets_dir() {
                                sync_dir(&cfg.get_app_assets_dir(), &custom_assets_dir);
                            }
                        }
                    }

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

fn sync_dir(dir: &Path, target: &Path) {
    dbg!(dir);
    dbg!(target);
    let d1 = std::fs::read_dir(dir)
        .unwrap()
        .map(|p| p.unwrap().file_name())
        .collect::<HashSet<_>>();
    let d2 = std::fs::read_dir(target)
        .unwrap()
        .map(|p| p.unwrap().file_name())
        .collect::<HashSet<_>>();

    // Copy over missing files to d2
    println!("\nCOPY");
    for filename in d1.difference(&d2) {
        //todo
        println!("{:?}", filename);
    }

    // Delete non-existent files in d2
    println!("\n\nDELETE");
    for filename in d2.difference(&d1) {
        //todo
        println!("{:?}", filename);
    }
}
