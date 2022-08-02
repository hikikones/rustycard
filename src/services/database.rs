use std::{collections::HashSet, ffi::OsString, path::Path, rc::Rc};

use rusqlite::{params, params_from_iter, Connection, Params, Row};

use super::config::Config;

const VERSION: usize = 1;

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
    pub due_date: chrono::NaiveDate,
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
        sync(SyncDirection::Open, &cfg);

        let conn = match Connection::open(&cfg.get_app_db_file()) {
            Ok(conn) => conn,
            Err(err) => panic!("{err}"),
        };

        let db = Self(Rc::new(conn));

        match db.get_version() {
            // New database
            0 => {
                db.write_batch(include_str!("schema.sql"));
                db.set_version(VERSION);
            }
            // Current version
            VERSION => {}
            // Unknown version
            _ => {
                panic!("Unknown database version");
            }
        }

        db
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

    fn last_insert_rowid(&self) -> Id {
        let id = self.0.last_insert_rowid();
        id.try_into().unwrap()
    }

    fn read_single<T, P>(&self, sql: &str, params: P) -> Option<T>
    where
        T: FromRow,
        P: Params,
    {
        self.0
            .query_row(sql, params, |row| Ok(T::from_row(row)))
            .ok()
    }

    fn read<T, P>(&self, sql: &str, params: P) -> Vec<T>
    where
        T: FromRow,
        P: Params,
    {
        let mut items = Vec::new();

        match self.0.prepare(sql) {
            Ok(mut stmt) => match stmt.query(params) {
                Ok(mut rows) => {
                    while let Ok(Some(row)) = rows.next() {
                        items.push(T::from_row(row));
                    }
                }
                Err(err) => panic!("{err}"),
            },
            Err(err) => panic!("{err}"),
        };

        items
    }

    fn write<P>(&self, sql: &str, params: P) -> usize
    where
        P: Params,
    {
        match self.0.execute(sql, params) {
            Ok(changed_rows) => changed_rows,
            Err(err) => panic!("{err}"),
        }
    }

    fn write_batch(&self, sql: &str) {
        self.0.execute_batch(sql).unwrap();
    }

    pub fn save(&self, cfg: &Config) {
        sync(SyncDirection::Close, &cfg);
    }

    fn get_version(&self) -> usize {
        self.read_single("SELECT user_version FROM pragma_user_version", [])
            .unwrap()
    }

    fn set_version(&self, version: usize) {
        self.write(&format!("PRAGMA user_version = {version}"), []);
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

impl FromRow for usize {
    fn from_row(row: &Row) -> Self {
        row.get(0).unwrap()
    }
}

enum SyncDirection {
    Open,
    Close,
}

fn sync(direction: SyncDirection, cfg: &Config) {
    if let Some(custom_db_file) = cfg.get_custom_db_file() {
        let db_file = match direction {
            SyncDirection::Open => (custom_db_file, cfg.get_app_db_file()),
            SyncDirection::Close => (cfg.get_app_db_file(), custom_db_file),
        };
        if sync_file(&db_file.0, &db_file.1) {
            if let Some(custom_assets_dir) = cfg.get_custom_assets_dir() {
                let assets_dir = match direction {
                    SyncDirection::Open => (custom_assets_dir, cfg.get_app_assets_dir()),
                    SyncDirection::Close => (cfg.get_app_assets_dir(), custom_assets_dir),
                };
                sync_dir(&assets_dir.0, &assets_dir.1);
            }
        }
    }
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
