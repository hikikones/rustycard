use std::{
    ops::Deref,
    path::{Path, PathBuf},
    rc::Rc,
};

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Clone)]
pub struct Config(Rc<ConfigData>);

#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    #[serde(skip)]
    app_path: PathBuf,
    version: usize,
    custom_db_file_path: Option<PathBuf>,
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            app_path: std::env::current_dir().unwrap(),
            version: 1,
            custom_db_file_path: None,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let mut cfg = ConfigData::default();
        let cfg_file = cfg.app_path.join(CONFIG_FILE_NAME);

        if !cfg_file.exists() {
            return Self(Rc::new(cfg));
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

        Self(Rc::new(cfg))
    }

    pub fn get_db_file_path(&self) -> PathBuf {
        self.custom_db_file_path
            .to_owned()
            .unwrap_or(self.get_app_db_file_path())
    }

    pub fn get_app_db_file_path(&self) -> PathBuf {
        self.app_path.join(DB_FILE_NAME)
    }

    pub fn get_custom_db_file_path(&self) -> Option<PathBuf> {
        self.custom_db_file_path.to_owned()
    }

    pub fn get_assets_dir_path(&self) -> PathBuf {
        self.app_path.join(ASSETS_DIR_NAME)
    }

    pub const fn get_assets_dir_name(&self) -> &str {
        ASSETS_DIR_NAME
    }
}

impl Deref for Config {
    type Target = ConfigData;

    fn deref(&self) -> &Self::Target {
        &self.0
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
