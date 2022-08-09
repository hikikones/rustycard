use std::{
    collections::HashSet,
    ffi::OsString,
    fs::File,
    io::{Seek, Write},
    path::Path,
    rc::Rc,
};

use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{params, params_from_iter, Connection, OpenFlags, Params, Row};

use super::{archive::*, config::Config};

pub type Id = usize;

#[derive(Clone)]
pub struct Database(Rc<Connection>);

#[derive(Debug)]
pub struct Card {
    pub id: Id,
    pub content: String,
    pub review: CardReview,
}

#[derive(Debug, Clone)]
pub struct CardReview {
    pub due_date: NaiveDate,
    pub due_days: usize,
    pub recall_attempts: usize,
    pub successful_recalls: usize,
}

#[derive(Debug)]
pub struct Tag {
    pub id: Id,
    pub name: String,
}

trait FromRow {
    fn from_row(row: &Row) -> Self;
}

impl Database {
    pub fn new(cfg: &Config) -> Self {
        // sync(SyncDirection::Open, &cfg);

        if let Some(location) = cfg.get_location() {
            if let Ok(file) = std::fs::File::open(location) {
                let mut reader = ZipArchive::new(file).unwrap();
                let bytes = reader.read_file(cfg.get_db_file_name());
                let mut kk = tempfile::NamedTempFile::new_in(cfg.get_app_dir()).unwrap();
                kk.write_all(&bytes).unwrap();
                // let mut tmp = tempfile::tempfile().unwrap();
                // tmp.write_all(&bytes).unwrap();
                // tmp.rewind().unwrap();

                // std::thread::sleep(std::time::Duration::from_secs(3));

                if Self::is_newer(kk.path(), &cfg.get_db_file()) {
                    reader.extract_file(cfg.get_db_file_name(), cfg.get_db_file(), true);
                    reader.extract_dir(cfg.get_assets_dir_name(), cfg.get_assets_dir(), false);
                }

                // let do_sync = {
                //     if !cfg.get_db_file().exists() {
                //         true
                //     } else {
                //         if true {
                //             false
                //         } else {
                //             false
                //         }
                //     }
                // };

                // if let Some(other_datetime) = Self::try_read_last_modified(kk.path()) {
                //     let do_sync = if let Some(local_datetime) =
                //         Self::try_read_last_modified(cfg.get_db_file())
                //     {
                //         other_datetime > local_datetime
                //     } else {
                //         false
                //     };
                // }

                // let d1 = Self::try_read_last_modified(cfg.get_db_file());
                // let d2 = Self::try_read_last_modified(kk.path());

                // dbg!(d1);
                // dbg!(d2);

                // let conn = match Connection::open(kk.path()) {
                //     Ok(conn) => conn,
                //     Err(err) => panic!("{err}"),
                // };

                // let db = Self(Rc::new(conn));

                // let d1 = self._get_last_modified();
                // let d2 = db._get_last_modified();

                // if d1 < d2 {
                //     //todo
                // }
            }
        }

        // TODO: Sync
        // if let Some(loc) = cfg.get_location() {
        //     if let Ok(file) = std::fs::File::open(loc) {
        //         let mut reader = ZipArchive::new(file).unwrap();
        //         let bytes = reader.read_file(cfg.get_db_file_name());
        //         // let mut tmp = tempfile::tempfile().unwrap();
        //         // tmp.write_all(&bytes).unwrap();
        //         // tmp.rewind().unwrap();

        //         let mut kk = tempfile::NamedTempFile::new_in(cfg.get_app_dir()).unwrap();
        //         kk.write_all(&bytes).unwrap();

        //         let conn = match Connection::open(kk.path()) {
        //             Ok(conn) => conn,
        //             Err(err) => panic!("{err}"),
        //         };

        //         let db = Self(Rc::new(conn));

        //         let d1 = self._get_last_modified();
        //         let d2 = db._get_last_modified();

        //         if d1 < d2 {
        //             //todo
        //         }
        //     }
        // }

        let conn = match Connection::open(&cfg.get_db_file()) {
            Ok(conn) => conn,
            Err(err) => panic!("{err}"),
        };

        let db = Self(Rc::new(conn));

        match db.try_get_version() {
            Some(version) => {
                match version {
                    // Current version
                    1 => {}
                    // Unknown version
                    _ => {
                        panic!("Unknown database version");
                    }
                }
            }
            None => {
                // New database
                db.write_batch(include_str!("schema.sql"));
            }
        }

        db
    }

    fn is_newer<P: AsRef<Path>>(db_file: P, other_db_file: P) -> bool {
        if !other_db_file.as_ref().exists() {
            return true;
        }

        if let Some(datetime) = Self::try_read_last_modified(db_file.as_ref()) {
            if let Some(other_datetime) = Self::try_read_last_modified(other_db_file.as_ref()) {
                return datetime > other_datetime;
            }
        }

        false
    }

    fn try_read_last_modified<P: AsRef<Path>>(db_file: P) -> Option<DateTime<Utc>> {
        let mut datetime = None;

        if let Ok(conn) = Connection::open_with_flags(db_file, OpenFlags::SQLITE_OPEN_READ_ONLY) {
            let db = Self(Rc::new(conn));
            datetime = db._try_get_last_modified();
        }

        datetime
    }

    pub fn _get_used_assets(&self, cfg: &Config) -> Vec<String> {
        let mut assets = Vec::new();

        // let cards = self.get_cards();
        let contents = self.read::<String, _>("SELECT content FROM cards", []);
        for entry in std::fs::read_dir(cfg.get_assets_dir()).unwrap() {
            if let Ok(asset) = entry {
                let file_name = asset.file_name();
                let name_lossy = file_name.to_string_lossy();
                for content in &contents {
                    if content.contains(name_lossy.as_ref()) {
                        assets.push(name_lossy.as_ref().to_owned());
                        break;
                    }
                }
                // self.read_with("SELECT content FROM cards", [], |row| {
                //     //todo
                //     let content: String = row.get(0).unwrap();
                //     // dbg!(content);
                //     if content.contains(name_lossy.as_ref()) {
                //         assets.push(name_lossy.as_ref().to_owned());
                //         return;
                //     }
                // });
                // for card in &cards {
                //     if card.content.contains(name_lossy.as_ref()) {
                //         assets.push(name_lossy.as_ref().to_owned());
                //         break;
                //     }
                // }
            }
        }

        assets
    }

    pub fn get_card(&self, id: Id) -> Card {
        assert!(id != 0);
        self.read_single("SELECT * FROM cards WHERE card_id = ?", [id])
            .unwrap()
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.read("SELECT * FROM cards", [])
    }

    pub fn get_cards_with_tags(&self, tags: &[Id]) -> Vec<Card> {
        if tags.is_empty() {
            return self.get_cards();
        }

        self.read(
            &format!(
                r#"
                SELECT * FROM cards
                JOIN card_tag USING (card_id)
                WHERE tag_id IN ({})
                GROUP BY card_id
                HAVING Count(*) = {}
                "#,
                tags.iter().map(|_| "?").collect::<Vec<_>>().join(","),
                tags.len()
            ),
            params_from_iter(tags.iter()),
        )
    }

    pub fn get_cards_without_tags(&self) -> Vec<Card> {
        self.read(
            r#"
                SELECT * FROM cards c WHERE NOT EXISTS (
                    SELECT ct.card_id FROM card_tag ct
                    WHERE c.card_id = ct.card_id
                )
                "#,
            [],
        )
    }

    pub fn _get_due_card_random(&self) -> Option<Card> {
        self.read_single(
            r#"
            SELECT * FROM cards
            WHERE due_date <= (date('now'))
            ORDER BY RANDOM()
            LIMIT 1
            "#,
            [],
        )
    }

    pub fn get_due_cards(&self) -> Vec<Card> {
        self.read(
            r#"
            SELECT * FROM cards
            WHERE due_date <= (date('now'))
            ORDER BY due_date ASC
            "#,
            [],
        )
    }

    pub fn _get_due_cards_count(&self) -> usize {
        self.read_single(
            r#"
            SELECT COUNT(card_id) FROM cards
            WHERE due_date <= (date('now'))
            "#,
            [],
        )
        .unwrap()
    }

    pub fn create_card(&self, content: &str) -> Id {
        self.write("INSERT INTO cards (content) VALUES (?)", [content]);
        self.last_insert_rowid()
    }

    pub fn update_card_content(&self, id: Id, content: &str) {
        assert!(id != 0);
        self.write(
            "UPDATE cards SET content = ? WHERE card_id = ?",
            params![content, id],
        );
    }

    pub fn update_card_review(&self, id: Id, review: CardReview) {
        assert!(id != 0);
        self.write(
            r#"
            UPDATE cards
            SET due_date = ?, due_days = ?, recall_attempts = ?, successful_recalls = ?
            WHERE card_id = ?
            "#,
            params![
                review.due_date,
                review.due_days,
                review.recall_attempts,
                review.successful_recalls,
                id
            ],
        );
    }

    pub fn _delete_card(&self, id: Id) {
        self.write("DELETE FROM cards WHERE card_id = ?", [id]);
    }

    pub fn _get_tag(&self, id: Id) -> Tag {
        assert!(id != 0);
        self.read_single("SELECT * FROM tags WHERE tag_id = ?", [id])
            .unwrap()
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        self.read("SELECT * FROM tags", [])
    }

    pub fn _create_tag(&self, name: &str) -> Id {
        self.write("INSERT INTO tags (name) VALUES (?)", [name]);
        self.last_insert_rowid()
    }

    pub fn _update_tag_name(&self, id: Id, name: &str) {
        assert!(id != 0);
        self.write(
            "UPDATE tags SET name = ? WHERE tag_id = ?",
            params![name, id],
        );
    }

    pub fn _delete_tag(&self, id: Id) {
        self.write("DELETE FROM tags WHERE tag_id = ?", [id]);
    }

    pub fn save(&self, cfg: &Config) {
        if let Some(location) = cfg.get_location() {
            // TODO: is_db_dirty?
            if let Ok(file) = File::create(location) {
                let mut writer = ZipWriter::new(file);
                writer.write_file(cfg.get_db_file(), cfg.get_db_file_name());
                writer.write_dir(cfg.get_assets_dir(), cfg.get_assets_dir_name());
            }
        }

        // sync(SyncDirection::Close, &cfg);
        // if let Some(loc) = cfg.get_location() {
        //     if let Ok(file) = std::fs::File::open(loc) {
        //         let mut zip = ZipArchive::new(file).unwrap();
        //         let bytes = zip.read_file(cfg.get_db_file_name());
        //         // let mut tmp = tempfile::tempfile().unwrap();
        //         // tmp.write_all(&bytes).unwrap();
        //         // tmp.rewind().unwrap();

        //         let mut kk = tempfile::NamedTempFile::new_in(cfg.get_app_dir()).unwrap();
        //         kk.write_all(&bytes).unwrap();

        //         let conn = match Connection::open(kk.path()) {
        //             Ok(conn) => conn,
        //             Err(err) => panic!("{err}"),
        //         };

        //         let db = Self(Rc::new(conn));

        //         let d1 = self._get_last_modified();
        //         let d2 = db._get_last_modified();

        //         if d1 < d2 {
        //             //todo
        //         }
        //     }
        // }
    }

    fn last_insert_rowid(&self) -> Id {
        let id = self.0.last_insert_rowid();
        id.try_into().unwrap()
    }

    fn try_get_version(&self) -> Option<usize> {
        let mut version = None;

        self.read_single_with(
            "SELECT metadata_id, version FROM metadata WHERE metadata_id = 1",
            [],
            |row| {
                version = Some(row.get(1).unwrap());
            },
        );

        version
    }

    fn _set_version(&self, version: usize) {
        self.write(
            "UPDATE metadata SET version = ? WHERE metadata_id = 1",
            params![version],
        );
    }

    fn _try_get_last_modified(&self) -> Option<DateTime<Utc>> {
        let mut datetime = None;

        self.read_single_with(
            "SELECT metadata_id, last_modified FROM metadata WHERE metadata_id = 1",
            [],
            |row| {
                datetime = Some(row.get(1).unwrap());
            },
        );

        datetime
    }

    fn _get_last_modified(&self) -> DateTime<Utc> {
        self._try_get_last_modified().unwrap()
    }

    fn update_last_modified(&self) {
        self.0
            .execute(
                "UPDATE metadata SET last_modified = (datetime('now')) WHERE metadata_id = 1",
                [],
            )
            .unwrap();
    }

    fn read_single_with<P, F>(&self, sql: &str, params: P, f: F)
    where
        P: Params,
        F: FnOnce(&Row),
    {
        self.0.query_row(sql, params, |row| Ok(f(row))).ok();
    }

    fn read_with<P, F>(&self, sql: &str, params: P, mut f: F)
    where
        P: Params,
        F: FnMut(&Row),
    {
        match self.0.prepare(sql) {
            Ok(mut stmt) => match stmt.query(params) {
                Ok(mut rows) => {
                    while let Ok(Some(row)) = rows.next() {
                        f(row);
                    }
                }
                Err(err) => panic!("{err}"),
            },
            Err(err) => panic!("{err}"),
        };
    }

    fn read_single<T, P>(&self, sql: &str, params: P) -> Option<T>
    where
        T: FromRow,
        P: Params,
    {
        let mut item = None;

        self.read_single_with(sql, params, |row| {
            item = Some(T::from_row(row));
        });

        item
    }

    fn read<T, P>(&self, sql: &str, params: P) -> Vec<T>
    where
        T: FromRow,
        P: Params,
    {
        let mut items = Vec::new();

        self.read_with(sql, params, |row| {
            items.push(T::from_row(row));
        });

        items
    }

    fn write<P>(&self, sql: &str, params: P) -> usize
    where
        P: Params,
    {
        let changed_rows = match self.0.execute(sql, params) {
            Ok(changed_rows) => changed_rows,
            Err(err) => panic!("{err}"),
        };

        self.update_last_modified();

        changed_rows
    }

    fn write_batch(&self, sql: &str) {
        self.0.execute_batch(sql).unwrap();
    }
}

impl FromRow for Card {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get(0).unwrap(),
            content: row.get(1).unwrap(),
            review: CardReview {
                due_date: row.get(2).unwrap(),
                due_days: row.get(3).unwrap(),
                recall_attempts: row.get(4).unwrap(),
                successful_recalls: row.get(5).unwrap(),
            },
        }
    }
}

impl FromRow for Tag {
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        }
    }
}

impl FromRow for String {
    fn from_row(row: &Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for usize {
    fn from_row(row: &Row) -> Self {
        row.get(0).unwrap()
    }
}

impl FromRow for DateTime<Utc> {
    fn from_row(row: &Row) -> Self {
        row.get(0).unwrap()
    }
}

enum SyncDirection {
    Open,
    Close,
}

fn sync(direction: SyncDirection, cfg: &Config) {
    // TODO

    // if let Some(custom_db_file) = cfg.get_custom_db_file() {
    //     let db_file = match direction {
    //         SyncDirection::Open => (custom_db_file, cfg.get_app_db_file()),
    //         SyncDirection::Close => (cfg.get_app_db_file(), custom_db_file),
    //     };
    //     if sync_file(&db_file.0, &db_file.1) {
    //         if let Some(custom_assets_dir) = cfg.get_custom_assets_dir() {
    //             let assets_dir = match direction {
    //                 SyncDirection::Open => (custom_assets_dir, cfg.get_app_assets_dir()),
    //                 SyncDirection::Close => (cfg.get_app_assets_dir(), custom_assets_dir),
    //             };
    //             sync_dir(&assets_dir.0, &assets_dir.1);
    //         }
    //     }
    // }
}

fn sync_file(file: &Path, other: &Path) -> bool {
    let is_newer = {
        if !other.exists() {
            return true;
        }

        let f1 = std::fs::File::open(file).unwrap();
        let f2 = std::fs::File::open(other).unwrap();

        let m1 = f1.metadata().unwrap();
        let m2 = f2.metadata().unwrap();

        m1.modified().unwrap() > m2.modified().unwrap()
    };

    if is_newer {
        std::fs::copy(file, other).unwrap();
    }

    is_newer
}

fn sync_dir(dir: &Path, other: &Path) {
    fn get_file_names(d: &Path) -> HashSet<OsString> {
        std::fs::read_dir(d)
            .unwrap()
            .map(|p| p.unwrap().file_name())
            .collect()
    }

    let d1 = get_file_names(dir);
    let d2 = get_file_names(other);

    // Copy over missing files
    for filename in d1.difference(&d2) {
        std::fs::copy(dir.join(filename), other.join(filename)).unwrap();
    }

    // Remove non-existent files
    for filename in d2.difference(&d1) {
        std::fs::remove_file(other.join(filename)).unwrap();
    }
}
