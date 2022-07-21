use std::path::PathBuf;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use toml::Value;

use services::{config::Config, database::Database};

mod components;
mod pages;
mod services;

struct AppConsts {
    config_file_name: &'static str,
    database_file_name: &'static str,
    assets_dir_name: &'static str,
}

impl AppConsts {
    #[cfg(not(debug_assertions))]
    const fn new() -> Self {
        Self {
            config_file_name: "config.toml",
            database_file_name: "rustycard.db",
            assets_dir_name: "assets",
        }
    }

    #[cfg(debug_assertions)]
    const fn new() -> Self {
        Self {
            config_file_name: "config.toml",
            database_file_name: "rustycard.db",
            assets_dir_name: "assets",
        }
    }
}

const APP_CONSTANTS: AppConsts = AppConsts::new();

struct DbFileName {
    name: &'static str,
}

const DB_FILE_NAME: DbFileName = DbFileName {
    name: "oejfoewijfw",
};

#[derive(Serialize, Deserialize)]
struct Cfg {
    version: usize,
    b: String,
}

fn main() {
    let c = Config::new();
    let s = toml::to_string(&*c).unwrap();
    std::fs::write("./config.toml", s).unwrap();

    // let cfg: Value = toml::from_str(
    //     r#"
    //     version = 1
    //     b = "b"
    // "#,
    // )
    // .unwrap();

    // // dbg!(&cfg.get(0));

    // match &cfg {
    //     Value::String(_) => todo!(),
    //     Value::Integer(_) => todo!(),
    //     Value::Float(_) => todo!(),
    //     Value::Boolean(_) => todo!(),
    //     Value::Datetime(_) => todo!(),
    //     Value::Array(_) => todo!(),
    //     Value::Table(table) => {
    //         //todo
    //         dbg!(table.contains_key("version"));
    //         dbg!(&table["version"]);
    //         dbg!(table.get("version"));
    //     }
    // }

    // let v = cfg
    //     .get(1)
    //     .unwrap()
    //     .as_str()
    //     .unwrap()
    //     .parse::<usize>()
    //     .unwrap();

    // let cfg: Cfg = toml::from_str(
    //     r#"
    //     version = 1
    //     b = "b"
    // "#,
    // )
    // .unwrap();

    // let v = cfg.version;

    // assert_eq!(v, 1);

    // assert_eq!(cfg["a"].as_str().unwrap(), "a");

    // Sync files from custom path
    {
        //todo
    }

    // dioxus::desktop::launch(app);

    // Sync files to custom path
    {
        //todo
    }
}

fn app(cx: Scope) -> Element {
    cx.use_hook(|_| {
        let cfg = Config::new();
        // let db = Database::new(cfg.get_db_file_path());
        // let a = DB_FILE_NAME.name.to_owned();
        // cx.provide_context(cfg);
        // cx.provide_context(db);
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
