use std::{fs::File, io::Write, path::Path};

use zip::{write::FileOptions, ZipArchive, ZipWriter};

trait ZipWriterExt {
    fn write_file(&mut self, file: &Path, name: &str);
    fn write_dir(&mut self, dir: &Path, name: &str);
}

trait ZipReaderExt {
    fn extract_file(&mut self, name: &str, target: &Path, replace: bool);
    fn extract_dir(&mut self, name: &str, target: &Path, replace: bool);
}

impl ZipWriterExt for ZipWriter<File> {
    fn write_file(&mut self, file: &Path, name: &str) {
        self.start_file(name, FileOptions::default()).unwrap();
        let bytes = std::fs::read(file).unwrap();
        self.write_all(&bytes).unwrap();
    }

    fn write_dir(&mut self, dir: &Path, name: &str) {
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
    fn extract_file(&mut self, name: &str, target: &Path, replace: bool) {
        if !replace && target.exists() {
            return;
        }

        if let Ok(mut zip_file) = self.by_name(name) {
            let mut target = std::fs::File::create(target).unwrap();
            std::io::copy(&mut zip_file, &mut target).unwrap();
        }
    }

    fn extract_dir(&mut self, name: &str, target: &Path, replace: bool) {
        if !target.is_dir() {
            panic!();
        }

        std::fs::create_dir_all(target).unwrap();
        let dir = Some(Path::new(name));

        for i in 0..self.len() {
            let mut zip_file = self.by_index(i).unwrap();

            let zip_path = match zip_file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            if zip_path.parent() == dir {
                let file_name = zip_path.file_name().unwrap();
                let file = target.join(file_name);

                if !replace && file.exists() {
                    continue;
                }

                let mut target = std::fs::File::create(file).unwrap();
                std::io::copy(&mut zip_file, &mut target).unwrap();
            }
        }
    }
}
