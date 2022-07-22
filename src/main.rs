use std::{
    any::Any,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

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
            config_file_name: "dev.toml",
            database_file_name: "dev.db",
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

thread_local! {
    static SERVICES: Mutex<Services> = Mutex::new(Services::new());
}

fn main() {
    dioxus::desktop::launch(test);

    // let c = Config::new();
    // let s = toml::to_string(&*c).unwrap();
    // std::fs::write("./config.toml", s).unwrap();

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

struct Yoyo;
impl Yoyo {
    fn yo(&self) {
        dbg!("yo!");
    }
}

fn test(cx: Scope) -> Element {
    cx.use_hook(|_| {
        SERVICES.with(|f| {
            let mut s = f.lock().unwrap();
            s.add(Config::new());
            s.add(Yoyo);
            s.add(Database::new(Path::new("dev.db")));
        });
    });

    cx.render(rsx! {
        button {
            onclick: |_| {
                SERVICES.with(|f| {
                    let s = f.lock().unwrap();
                    // s.add(Config::new());
                    s.get::<Yoyo>().yo();
                    let db = s.get::<Database>();
                    dbg!(db.get_cards());
                });
            },
            "Yo"
        }
        button {
            onclick: |_| {
                SERVICES.with(|f| {
                    let s = f.lock().unwrap();
                    // s.add(Config::new());
                    let y = s.get::<Yoyo>();
                    let db = s.get::<Database>();
                    dbg!(db.get_cards());
                    y.yo();
                });
            },
            "Yo2"
        }
    })
}

pub struct Services {
    items: HashMap<String, Box<dyn Any>>,
}

impl Services {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add<T: Any>(&mut self, t: T) {
        let name = std::any::type_name::<T>();
        println!("NAME: {}", name);
        self.items.insert(name.to_string(), Box::new(t));
    }

    pub fn get<T: Any>(&self) -> &T {
        let name = std::any::type_name::<T>();
        let s = self.items.get(name).unwrap();

        let t = match s.downcast_ref::<T>() {
            Some(a) => a,
            None => panic!("NO SERVICE!!!"),
        };

        t
    }
}
