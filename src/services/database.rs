use std::{cell::RefCell, path::Path, rc::Rc};

use rusqlite::{params, params_from_iter, Connection, Error, Params, Row};

const VERSION: usize = 1;

pub type Id = usize;

#[derive(Clone)]
pub struct Database(Rc<RefCell<Rusqlite>>);

pub struct Rusqlite {
    connection: Connection,
    is_dirty: bool,
}

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

impl Database {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let conn = match Connection::open(&path) {
            Ok(conn) => conn,
            Err(err) => panic!("{err}"),
        };

        let db = Self(Rc::new(RefCell::new(Rusqlite {
            connection: conn,
            is_dirty: false,
        })));

        match db.get_version() {
            0 => {
                // New database
                db.write_batch(include_str!("schema.sql"));
                db.set_version(VERSION);
            }
            VERSION => {
                // Current version
                println!("\nCURRENT!\n");
            }
            _ => {
                // Unknown version
                panic!("Unknown database version");
            }
        }

        db
    }

    pub fn get_card(&self, id: Id) -> Card {
        assert!(id != 0);
        self.read_single("SELECT * FROM cards WHERE card_id = ?", [id], |row| {
            row.into()
        })
        .unwrap()
    }

    pub fn get_cards(&self) -> Vec<Card> {
        let mut cards = Vec::new();
        self.read("SELECT * FROM cards", [], |row| {
            cards.push(row.into());
        });
        cards
    }

    pub fn get_cards_with_tags(&self, tags: &[Id]) -> Vec<Card> {
        if tags.is_empty() {
            return self.get_cards();
        }

        let mut cards = Vec::new();
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
            |row| {
                cards.push(row.into());
            },
        );
        cards
    }

    pub fn get_cards_without_tags(&self) -> Vec<Card> {
        let mut cards = Vec::new();
        self.read(
            r#"
                SELECT * FROM cards c WHERE NOT EXISTS (
                    SELECT ct.card_id FROM card_tag ct
                    WHERE c.card_id = ct.card_id
                )
                "#,
            [],
            |row| {
                cards.push(row.into());
            },
        );
        cards
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
            |row| row.into(),
        )
        .ok()
    }

    pub fn get_due_cards(&self) -> Vec<Card> {
        let mut cards = Vec::new();
        self.read(
            r#"
            SELECT * FROM cards
            WHERE due_date <= (date('now'))
            ORDER BY due_date ASC
            "#,
            [],
            |row| {
                cards.push(row.into());
            },
        );
        cards
    }

    pub fn _get_due_cards_count(&self) -> usize {
        self.read_single(
            r#"
            SELECT COUNT(card_id) FROM cards
            WHERE due_date <= (date('now'))
            "#,
            [],
            |row| row.get(0).unwrap(),
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
        self.read_single("SELECT * FROM tags WHERE tag_id = ?", [id], |row| {
            row.into()
        })
        .unwrap()
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        let mut tags = Vec::new();
        self.read("SELECT * FROM tags", [], |row| {
            tags.push(row.into());
        });
        tags
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
        let id = self.0.borrow().connection.last_insert_rowid();
        id.try_into().unwrap()
    }

    fn read_single<T>(
        &self,
        sql: &str,
        params: impl Params,
        f: impl FnOnce(&Row) -> T,
    ) -> Result<T, Error> {
        self.0
            .borrow()
            .connection
            .query_row(sql, params, |row| Ok(f(row)))
    }

    fn read(&self, sql: &str, params: impl Params, mut f: impl FnMut(&Row)) {
        match self.0.borrow().connection.prepare(sql) {
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

    fn write(&self, sql: &str, params: impl Params) -> usize {
        let rows = match self.0.borrow().connection.execute(sql, params) {
            Ok(changed_rows) => changed_rows,
            Err(err) => panic!("{err}"),
        };
        self.0.borrow_mut().is_dirty = true;
        rows
    }

    fn write_batch(&self, sql: &str) {
        self.0.borrow().connection.execute_batch(sql).unwrap();
        self.0.borrow_mut().is_dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.0.borrow().is_dirty
    }

    fn get_version(&self) -> usize {
        self.read_single("SELECT user_version FROM pragma_user_version", [], |row| {
            row.get(0).unwrap()
        })
        .unwrap()
    }

    fn set_version(&self, version: usize) {
        self.write(&format!("PRAGMA user_version = {version}"), []);
    }
}

impl From<&Row<'_>> for Card {
    fn from(row: &Row) -> Self {
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

impl From<&Row<'_>> for Tag {
    fn from(row: &Row<'_>) -> Self {
        Self {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        }
    }
}
