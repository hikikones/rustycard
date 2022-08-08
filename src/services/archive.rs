use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use zip::write::FileOptions;

pub use zip::{ZipArchive, ZipWriter};

pub trait ZipWriterExt {
    fn write_file<P: AsRef<Path>>(&mut self, file: P, name: &str);
    fn write_dir<P: AsRef<Path>>(&mut self, dir: P, name: &str);
}

pub trait ZipReaderExt {
    fn read_file(&mut self, name: &str) -> Vec<u8>;
    fn extract_file<P: AsRef<Path>>(&mut self, name: &str, target: P, replace: bool);
    fn extract_dir<P: AsRef<Path>>(&mut self, name: &str, target: P, replace: bool);
}

impl ZipWriterExt for ZipWriter<File> {
    fn write_file<P: AsRef<Path>>(&mut self, file: P, name: &str) {
        self.start_file(name, FileOptions::default()).unwrap();
        let bytes = std::fs::read(file).unwrap();
        self.write_all(&bytes).unwrap();
    }

    fn write_dir<P: AsRef<Path>>(&mut self, dir: P, name: &str) {
        self.add_directory(name, FileOptions::default()).unwrap();
        for entry in std::fs::read_dir(dir).unwrap() {
            let file_path = entry.unwrap().path();
            let file_name = file_path.file_name().unwrap();
            let target = Path::new(name)
                .join(file_name)
                .to_string_lossy()
                .replace("\\", "/");
            self.write_file(&file_path, &target);
        }
    }
}

impl ZipReaderExt for ZipArchive<File> {
    fn read_file(&mut self, name: &str) -> Vec<u8> {
        self.by_name(name)
            .unwrap()
            .bytes()
            .filter_map(|b| b.ok())
            .collect()
    }

    fn extract_file<P: AsRef<Path>>(&mut self, name: &str, target: P, replace: bool) {
        if !replace && target.as_ref().exists() {
            return;
        }

        if let Ok(mut zip_file) = self.by_name(name) {
            let mut target = std::fs::File::create(target).unwrap();
            std::io::copy(&mut zip_file, &mut target).unwrap();
        }
    }

    fn extract_dir<P: AsRef<Path>>(&mut self, name: &str, target: P, replace: bool) {
        if !target.as_ref().is_dir() {
            panic!();
        }

        std::fs::create_dir_all(target.as_ref()).unwrap();
        let dir = Some(Path::new(name));

        for i in 0..self.len() {
            let mut zip_file = self.by_index(i).unwrap();

            let zip_path = match zip_file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            if zip_path.parent() == dir {
                let file_name = zip_path.file_name().unwrap();
                let file = target.as_ref().join(file_name);

                if !replace && file.exists() {
                    continue;
                }

                let mut target = std::fs::File::create(file).unwrap();
                std::io::copy(&mut zip_file, &mut target).unwrap();
            }
        }
    }
}
