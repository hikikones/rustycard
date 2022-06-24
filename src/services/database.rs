use std::{ops::Add, path::Path, rc::Rc};

use chrono::{Date, DateTime, Datelike, NaiveDate, Utc};
use rusqlite::{params, params_from_iter, Connection, OpenFlags, Params, Row};

#[derive(Clone)]
pub struct Database {
    connection: Rc<Connection>,
}

impl Database {
    pub fn new(path: impl AsRef<Path>) -> Self {
        match Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_WRITE) {
            Ok(conn) => Self {
                connection: Rc::new(conn),
            },
            Err(_) => {
                // Database not found. Create one.
                match Connection::open(&path) {
                    Ok(conn) => {
                        // TODO: Assets path.
                        conn.execute_batch(include_str!("schema.sql")).unwrap();
                        Self {
                            connection: Rc::new(conn),
                        }
                    }
                    Err(err) => {
                        panic!("Error opening database: {}", err)
                    }
                }
            }
        }
    }

    pub fn get_card(&self, id: usize) -> Card {
        assert!(id != 0);
        self.read_single("SELECT * FROM cards WHERE card_id = ?", [id])
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.read_many("SELECT * FROM cards", [])
    }

    pub fn _get_due_card(&self) -> Option<Card> {
        // FIXME
        match self.connection.query_row(
            "SELECT * FROM cards ORDER BY RANDOM() LIMIT 1",
            [],
            |row| Ok(<Card as DbItem>::from(row)),
        ) {
            Ok(card) => Some(card),
            Err(_) => None,
        }
    }

    pub fn get_due_cards(&self) -> Vec<Card> {
        // FIXME
        self.get_cards()
    }

    pub fn _get_due_cards_count(&self) -> usize {
        // FIXME
        match self
            .connection
            .query_row("SELECT COUNT(card_id) FROM cards", [], |row| row.get(0))
        {
            Ok(item) => item,
            Err(err) => panic!("Error query row: {}", err),
        }
    }

    pub fn create_card(&self, content: &str) -> usize {
        self.write_single("INSERT INTO cards (content) VALUES (?)", [content])
    }

    pub fn update_card_content(&self, id: usize, content: &str) {
        assert!(id != 0);
        self.write_single(
            "UPDATE cards SET content = ? WHERE card_id = ?",
            params![content, id],
        );
    }

    pub fn _get_tag(&self, id: usize) -> Tag {
        assert!(id != 0);
        self.read_single("SELECT * FROM tags WHERE tag_id = ?", [id])
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        self.read_many("SELECT * FROM tags", [])
    }

    pub fn _create_tag(&self, name: &str) -> usize {
        self.write_single("INSERT INTO tags (name) VALUES (?)", [name])
    }

    pub fn _update_tag_name(&self, id: usize, name: &str) {
        assert!(id != 0);
        self.write_single(
            "UPDATE tags SET name = ? WHERE tag_id = ?",
            params![name, id],
        );
    }

    pub fn get_cards_by_tags(&self, tags: &[usize]) -> Vec<Card> {
        if tags.is_empty() {
            return self.get_cards();
        }

        self.read_many(
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
        self.read_many(
            r#"
                SELECT * FROM cards c WHERE NOT EXISTS (
                    SELECT ct.card_id FROM card_tag ct
                    WHERE c.card_id = ct.card_id
                )
                "#,
            [],
        )
    }

    fn read_single<T: DbItem>(&self, sql: &str, params: impl Params) -> T {
        match self
            .connection
            .query_row(sql, params, move |row| Ok(T::from(row)))
        {
            Ok(item) => item,
            Err(err) => panic!("Error query row: {}", err),
        }
    }

    fn read_many<T: DbItem>(&self, sql: &str, params: impl Params) -> Vec<T> {
        let mut stmt = match self.connection.prepare(sql) {
            Ok(stmt) => stmt,
            Err(err) => panic!("Error preparing query: {}", err),
        };
        let rows = match stmt.query_map(params, |row| Ok(T::from(row))) {
            Ok(rows) => rows,
            Err(err) => panic!("Error query map: {}", err),
        };

        rows.filter_map(|item| item.ok()).collect()
    }

    fn write_single(&self, sql: &str, params: impl Params) -> usize {
        match self.connection.execute(sql, params) {
            Ok(id) => id,
            Err(err) => panic!("Error execute query: {}", err),
        }
    }
}

pub trait DbItem {
    fn from(row: &Row) -> Self;
}

pub struct Card {
    pub id: usize,
    pub content: String,
    pub due_date: DateTime<Utc>,
    // TODO
}

impl DbItem for Card {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0).unwrap(),
            content: row.get(1).unwrap(),
            // due_date: row
            //     .get::<_, String>(2)
            //     .unwrap()
            //     .parse::<NaiveDate>()
            //     .unwrap(),
            due_date: row.get(2).unwrap(),
        }
    }
}

pub struct Tag {
    pub id: usize,
    pub name: String,
    // TODO
}

impl DbItem for Tag {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        }
    }
}
