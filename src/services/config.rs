use std::{
    ops::Deref,
    path::{Path, PathBuf},
    rc::Rc,
};

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::APP_CONSTANTS;

// #[cfg(not(debug_assertions))]
// {
//     pub const CONFIG_FILE_NAME: &'static str = "config.toml";
//     pub const AAAA: &'static str = "config.toml";

// }
// #[cfg(debug_assertions)]
// pub const CONFIG_FILE_NAME: &'static str = "dev.toml";

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

impl Config {
    pub fn new() -> Self {
        let path = Path::new(APP_CONSTANTS.config_file_name);
        if !path.exists() {
            return Self(Rc::new(ConfigData::default()));
        }

        let data = std::fs::read_to_string(path).unwrap();
        let value: Value = toml::from_str(&data).unwrap();
        let version = match value {
            Value::Table(table) => {
                if !table.contains_key("version") {
                    panic!(); // todo
                }
                table["version"].as_integer().unwrap()
            }
            _ => {
                panic!() // todo
            }
        };

        match version {
            0 => {
                #[derive(Deserialize)]
                struct V0 {
                    location: Option<PathBuf>,
                }
                let cfg: V0 = toml::from_str(&data).unwrap();
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

        // let app_path = get_app_path();
        // let db_file_name = get_db_file_name();
        // let assets_dir_name = "assets".into();

        // std::fs::create_dir_all(&app_path.join(&assets_dir_name)).unwrap();

        // Self(Rc::new(ConfigData {
        //     app_path,
        //     db_file_name,
        //     assets_dir_name,
        // }))
    }

    // pub fn get_assets_dir_name(&self) -> &str {
    //     &self.assets_dir_name
    // }

    // pub fn get_db_file_path(&self) -> PathBuf {
    //     self.app_path.join(&self.db_file_name)
    // }

    // pub fn get_assets_dir_path(&self) -> PathBuf {
    //     self.app_path.join(&self.assets_dir_name)
    // }
}

impl Deref for Config {
    type Target = ConfigData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(debug_assertions)]
fn get_app_path() -> PathBuf {
    ".".into()
}

#[cfg(not(debug_assertions))]
fn get_app_path() -> PathBuf {
    // let app_dirs = platform_dirs::AppDirs::new(Some("rustycard"), false).unwrap();
    // app_dirs.data_dir
    ".".into()
}

#[cfg(debug_assertions)]
fn get_db_file_name() -> String {
    "dev.db".into()
}

#[cfg(not(debug_assertions))]
fn get_db_file_name() -> String {
    "rustycard.db".into()
}
