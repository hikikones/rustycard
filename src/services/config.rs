use std::{
    cell::RefCell,
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
};

use dioxus::prelude::ScopeState;
use serde::{Deserialize, Serialize};
use toml::Value;

pub fn use_config(cx: &ScopeState) -> &RefCell<Config> {
    &*cx.use_hook(|_| cx.consume_context::<Rc<RefCell<Config>>>().unwrap())
}

#[derive(Clone)]
pub struct Config(Rc<RefCell<ConfigData>>);

#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    version: usize,
    location: Option<PathBuf>,

    #[serde(skip)]
    app_dir: PathBuf,
    #[serde(skip)]
    is_dirty: bool,
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            version: 1,
            location: None,
            app_dir: std::env::current_dir().unwrap(),
            is_dirty: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let mut cfg = ConfigData::default();

        let cfg_file = cfg.get_config_file();
        let app_assets_dir = cfg.get_assets_dir();

        if !app_assets_dir.exists() {
            std::fs::create_dir(app_assets_dir).unwrap();
        }

        if !cfg_file.exists() {
            return Self(Rc::new(RefCell::new(cfg)));
        }

        let data = std::fs::read_to_string(cfg_file).unwrap();
        let value: Value = toml::from_str(&data).unwrap();

        if let Value::Table(table) = value {
            if let Some(version) = table.get("version") {
                if let Some(version) = version.as_integer() {
                    match version {
                        1 => {
                            if let Some(location) = table.get("location") {
                                if let Some(location) = location.as_str() {
                                    let path = PathBuf::from(location);
                                    if path.exists() && path.is_file() {
                                        cfg.location = Some(path);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Self(Rc::new(RefCell::new(cfg)))
    }

    pub fn get_app_dir(&self) -> PathBuf {
        self.0.borrow().app_dir.to_owned()
    }

    pub fn get_db_file_name(&self) -> &str {
        DB_FILE_NAME
    }

    pub fn get_db_file(&self) -> PathBuf {
        self.0.borrow().app_dir.join(DB_FILE_NAME)
    }

    pub fn get_assets_dir(&self) -> PathBuf {
        self.0.borrow().get_assets_dir()
    }

    pub fn get_location(&self) -> Option<PathBuf> {
        self.0.borrow().location.to_owned()
    }

    pub fn set_location(&self, path: &Path) {
        let mut data = self.0.borrow_mut();
        data.location = Some(path.to_owned());
        data.is_dirty = true;
    }

    pub const fn get_assets_dir_name(&self) -> &str {
        ASSETS_DIR_NAME
    }

    pub fn save(&self) {
        let data = self.0.borrow();
        if data.is_dirty {
            let toml = toml::to_string(&*data).unwrap();
            let mut file = std::fs::File::create(data.get_config_file()).unwrap();
            file.write_all(toml.as_bytes()).unwrap();
        }
    }
}

impl ConfigData {
    fn get_config_file(&self) -> PathBuf {
        self.app_dir.join(CONFIG_FILE_NAME)
    }

    fn get_assets_dir(&self) -> PathBuf {
        self.app_dir.join(ASSETS_DIR_NAME)
    }
}

#[cfg(debug_assertions)]
const CONFIG_FILE_NAME: &str = "dev.toml";

#[cfg(not(debug_assertions))]
const CONFIG_FILE_NAME: &str = "config.toml";

#[cfg(debug_assertions)]
const DB_FILE_NAME: &str = "dev.db";

#[cfg(not(debug_assertions))]
const DB_FILE_NAME: &str = "rustycard.db";

const ASSETS_DIR_NAME: &str = "assets";
