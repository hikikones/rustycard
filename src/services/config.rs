use std::{ops::Deref, path::PathBuf, rc::Rc};

#[derive(Clone)]
pub struct Config(Rc<ConfigData>);

pub struct ConfigData {
    pub db_file: PathBuf,
    pub assets_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let app_dir = get_app_dir();
        let db_file = app_dir.join("rustycard.db");
        let assets_dir = app_dir.join("assets/");

        std::fs::create_dir_all(&assets_dir).unwrap();

        Self(Rc::new(ConfigData {
            db_file,
            assets_dir,
        }))
    }
}

impl Deref for Config {
    type Target = ConfigData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(debug_assertions)]
fn get_app_dir() -> PathBuf {
    ".".into()
}

#[cfg(not(debug_assertions))]
fn get_app_dir() -> PathBuf {
    let app_dirs = platform_dirs::AppDirs::new(Some("rustycard"), false).unwrap();
    app_dirs.data_dir
}
