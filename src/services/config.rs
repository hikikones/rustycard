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
    version: usize,
    pub location: Option<PathBuf>,
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            version: 1,
            location: None,
        }
    }
}

// TODO: Handle panics.
impl Config {
    pub fn new() -> Self {
        let cfg_file = get_app_path().join(get_config_file_name());

        if !cfg_file.exists() {
            return Self(Rc::new(ConfigData::default()));
        }

        let data = std::fs::read_to_string(cfg_file).unwrap();
        let value: Value = toml::from_str(&data).unwrap();
        let version = match value {
            Value::Table(table) => {
                if !table.contains_key("version") {
                    panic!();
                }
                table["version"].as_integer().unwrap()
            }
            _ => {
                panic!()
            }
        };

        match version {
            0 => {
                #[derive(Deserialize)]
                struct ConfigV0 {
                    location: Option<PathBuf>,
                }
                let cfg: ConfigV0 = toml::from_str(&data).unwrap();
                Self(Rc::new(ConfigData {
                    location: cfg.location,
                    ..Default::default()
                }))
            }
            1 => {
                let cfg: ConfigData = toml::from_str(&data).unwrap();
                Self(Rc::new(cfg))
            }
            _ => panic!(),
        }
    }

    pub fn get_db_file_path(&self) -> PathBuf {
        get_app_path().join(get_db_file_name())
    }

    pub fn get_assets_dir_path(&self) -> PathBuf {
        get_app_path().join(self.get_assets_dir_name())
    }

    pub const fn get_assets_dir_name(&self) -> &str {
        "assets"
    }
}

impl Deref for Config {
    type Target = ConfigData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn get_app_path() -> &'static Path {
    Path::new(".")
}

#[cfg(debug_assertions)]
const fn get_config_file_name() -> &'static str {
    "dev.toml"
}

#[cfg(not(debug_assertions))]
const fn get_config_file_name() -> &'static str {
    "config.toml"
}

#[cfg(debug_assertions)]
const fn get_db_file_name() -> &'static str {
    "dev.db"
}

#[cfg(not(debug_assertions))]
const fn get_db_file_name() -> &'static str {
    "rustycard.db"
}
