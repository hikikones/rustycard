use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
};

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Clone)]
pub struct Config {
    data: Rc<RefCell<ConfigData>>,
    is_dirty: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    #[serde(skip)]
    app_path: PathBuf,
    version: usize,
    custom_db_file_path: Option<PathBuf>,
    custom_assets_dir_path: Option<PathBuf>,
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            app_path: std::env::current_dir().unwrap(),
            version: 1,
            custom_db_file_path: None,
            custom_assets_dir_path: None,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let mut cfg = ConfigData::default();
        let cfg_file = cfg.app_path.join(CONFIG_FILE_NAME);

        if !cfg_file.exists() {
            return Self {
                data: Rc::new(RefCell::new(cfg)),
                is_dirty: false,
            };
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
                                    cfg.custom_db_file_path = Some(PathBuf::from(location));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Self {
            data: Rc::new(RefCell::new(cfg)),
            is_dirty: false,
        }
    }

    pub fn get_db_file_path(&self) -> PathBuf {
        self.data
            .borrow()
            .custom_db_file_path
            .to_owned()
            .unwrap_or(self.get_app_db_file_path())
    }

    pub fn get_app_db_file_path(&self) -> PathBuf {
        self.data.borrow().app_path.join(DB_FILE_NAME)
    }

    pub fn get_custom_db_file_path(&self) -> Option<PathBuf> {
        self.data.borrow().custom_db_file_path.to_owned()
    }

    pub fn set_custom_db_file_path(&mut self, path: impl AsRef<Path>) {
        assert!(path.as_ref().is_file());
        self.data.borrow_mut().custom_db_file_path = Some(path.as_ref().to_path_buf());
        self.is_dirty = true;
    }

    pub fn get_assets_dir_path(&self) -> PathBuf {
        self.data
            .borrow()
            .custom_assets_dir_path
            .to_owned()
            .unwrap_or(self.get_app_assets_dir_path())
    }

    pub fn get_app_assets_dir_path(&self) -> PathBuf {
        self.data.borrow().app_path.join(ASSETS_DIR_NAME)
    }

    pub fn get_custom_assets_dir_path(&self) -> Option<PathBuf> {
        self.data.borrow().custom_assets_dir_path.to_owned()
    }

    pub fn set_custom_assets_dir_path(&mut self, path: impl AsRef<Path>) {
        assert!(path.as_ref().is_dir());
        self.data.borrow_mut().custom_assets_dir_path = Some(path.as_ref().to_path_buf());
        self.is_dirty = true;
    }

    pub const fn get_assets_dir_name(&self) -> &str {
        ASSETS_DIR_NAME
    }

    pub const fn is_dirty(&self) -> bool {
        self.is_dirty
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
