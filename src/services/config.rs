use std::{ops::Deref, path::PathBuf, rc::Rc};

#[cfg(not(debug_assertions))]
pub const CONFIG_FILE_NAME: &'static str = "config.toml";
#[cfg(debug_assertions)]
pub const CONFIG_FILE_NAME: &'static str = "dev.toml";

#[derive(Clone)]
pub struct Config(Rc<ConfigData>);

pub struct ConfigData {
    app_path: PathBuf,
    db_file_name: String,
    assets_dir_name: String,
}

impl Config {
    pub fn new() -> Self {
        let app_path = get_app_path();
        let db_file_name = get_db_file_name();
        let assets_dir_name = "assets".into();

        std::fs::create_dir_all(&app_path.join(&assets_dir_name)).unwrap();

        Self(Rc::new(ConfigData {
            app_path,
            db_file_name,
            assets_dir_name,
        }))
    }

    pub fn get_assets_dir_name(&self) -> &str {
        &self.assets_dir_name
    }

    pub fn get_db_file_path(&self) -> PathBuf {
        self.app_path.join(&self.db_file_name)
    }

    pub fn get_assets_dir_path(&self) -> PathBuf {
        self.app_path.join(&self.assets_dir_name)
    }
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
