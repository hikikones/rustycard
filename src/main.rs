use std::{cell::RefCell, rc::Rc};

use dioxus::{desktop::use_window, prelude::*};

use services::{
    config::{use_config, Config},
    database::{use_database, Database},
};

use crate::components::Button;

mod components;
mod pages;
mod services;

fn main() {
    dioxus::desktop::launch_cfg(app, |c| {
        let head = format!(
            r#"
            <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css">
            <style>{}</style>
        "#,
            include_str!("components/button.css")
        );
        c.with_custom_head(head)
    });
}

fn app(cx: Scope) -> Element {
    cx.use_hook(|_| {
        let cfg = Config::new();
        let db = Database::new(&cfg);
        cx.provide_context(Rc::new(RefCell::new(cfg)));
        cx.provide_context(Rc::new(RefCell::new(db)));
    });

    let cfg = use_config(&cx);
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
                    cfg.borrow().save();
                    db.borrow().save(&*cfg.borrow());
                    window.close();
                },
                "Quit"
            }
            Button {
                onclick: |_| {
                    println!("YOYO");
                }
                name: "YOYO",
                disabled: true,
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
