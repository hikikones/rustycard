use std::{
    cell::RefCell,
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
};

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Clone)]
pub struct Config(Rc<RefCell<ConfigData>>);

#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    #[serde(skip)]
    app_path: PathBuf,
    version: usize,
    db_file: Option<PathBuf>,
    assets_dir: Option<PathBuf>,
    #[serde(skip)]
    is_dirty: bool,
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            app_path: std::env::current_dir().unwrap(),
            version: 1,
            db_file: None,
            assets_dir: None,
            is_dirty: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let mut cfg = ConfigData::default();

        let cfg_file = cfg.get_app_config_file();
        let app_assets_dir = cfg.get_app_assets_dir();

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
                            if let Some(db_file) = table.get("db_file") {
                                if let Some(db_file) = db_file.as_str() {
                                    let path = PathBuf::from(db_file);
                                    if path.exists() && path.is_file() {
                                        cfg.db_file = Some(path);
                                    }
                                }
                            }

                            if let Some(assets_dir) = table.get("assets_dir") {
                                if let Some(assets_dir) = assets_dir.as_str() {
                                    let path = PathBuf::from(assets_dir);
                                    if path.exists() && path.is_dir() {
                                        cfg.assets_dir = Some(path);
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

    pub fn get_db_file(&self) -> PathBuf {
        self.0
            .borrow()
            .db_file
            .to_owned()
            .unwrap_or(self.get_app_db_file())
    }

    pub fn get_app_db_file(&self) -> PathBuf {
        self.0.borrow().app_path.join(DB_FILE_NAME)
    }

    pub fn get_custom_db_file(&self) -> Option<PathBuf> {
        self.0.borrow().db_file.to_owned()
    }

    pub fn set_custom_db_file(&self, path: impl AsRef<Path>) {
        let mut data = self.0.borrow_mut();
        data.db_file = Some(path.as_ref().to_path_buf());
        data.is_dirty = true;
    }

    pub fn get_assets_dir(&self) -> PathBuf {
        self.0
            .borrow()
            .assets_dir
            .to_owned()
            .unwrap_or(self.get_app_assets_dir())
    }

    pub fn get_app_assets_dir(&self) -> PathBuf {
        self.0.borrow().get_app_assets_dir()
    }

    pub fn get_custom_assets_dir(&self) -> Option<PathBuf> {
        self.0.borrow().assets_dir.to_owned()
    }

    pub fn set_custom_assets_dir(&self, path: impl AsRef<Path>) {
        let mut data = self.0.borrow_mut();
        data.assets_dir = Some(path.as_ref().to_path_buf());
        data.is_dirty = true;
    }

    pub const fn get_assets_dir_name(&self) -> &str {
        ASSETS_DIR_NAME
    }

    pub fn save(&self) {
        let data = self.0.borrow();
        if data.is_dirty {
            let toml = toml::to_string(&*data).unwrap();
            let mut file = std::fs::File::create(data.get_app_config_file()).unwrap();
            file.write_all(toml.as_bytes()).unwrap();
        }
    }
}

impl ConfigData {
    fn get_app_config_file(&self) -> PathBuf {
        self.app_path.join(CONFIG_FILE_NAME)
    }

    fn get_app_assets_dir(&self) -> PathBuf {
        self.app_path.join(ASSETS_DIR_NAME)
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
