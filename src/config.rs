use std::{ops::Deref, path::PathBuf, rc::Rc};

#[derive(Clone)]
pub struct Config(Rc<Cfg>);

pub struct Cfg {
    pub db_file: PathBuf,
    pub media_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let app_dirs = platform_dirs::AppDirs::new(Some("rustycard"), false).unwrap();
        std::fs::create_dir_all(&app_dirs.data_dir).unwrap();

        let db_file = app_dirs.data_dir.join("database.sqlite3");
        let media_dir = app_dirs.data_dir.join("media/");
        std::fs::create_dir_all(&media_dir).unwrap();

        // TODO: Parse or create config file.
        // let file = if config_file_path.exists() {
        //     File::open(config_file_path).unwrap()
        // } else {
        //     File::create(config_file_path).unwrap()
        // };

        Self(Rc::new(Cfg { db_file, media_dir }))
    }
}

impl Deref for Config {
    type Target = Cfg;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
