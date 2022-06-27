use std::{path::Path, rc::Rc};

use rusqlite::{params, params_from_iter, Connection, Error, OpenFlags, Params, Row};

pub type Id = usize;

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
        let id = self.connection.last_insert_rowid();
        id.try_into().unwrap()
    }

    // fn read_single<T: DbItem>(&self, sql: &str, params: impl Params) -> T {
    //     match self
    //         .connection
    //         .query_row(sql, params, move |row| Ok(T::from(row)))
    //     {
    //         Ok(item) => item,
    //         Err(err) => panic!("Error query row: {}", err),
    //     }
    // }

    // fn read_many<T: DbItem>(&self, sql: &str, params: impl Params) -> Vec<T> {
    //     let mut stmt = match self.connection.prepare(sql) {
    //         Ok(stmt) => stmt,
    //         Err(err) => panic!("Error preparing query: {}", err),
    //     };
    //     let rows = match stmt.query_map(params, |row| Ok(T::from(row))) {
    //         Ok(rows) => rows,
    //         Err(err) => panic!("Error query map: {}", err),
    //     };

    //     rows.filter_map(|item| item.ok()).collect()
    // }

    // fn write_single(&self, sql: &str, params: impl Params) -> Id {
    //     match self.connection.execute(sql, params) {
    //         Ok(id) => id,
    //         Err(err) => panic!("Error execute query: {}", err),
    //     }
    // }

    fn read_single<T>(
        &self,
        sql: &str,
        params: impl Params,
        f: impl FnOnce(&Row) -> T,
    ) -> Result<T, Error> {
        self.connection.query_row(sql, params, |row| Ok(f(row)))
    }

    fn read(&self, sql: &str, params: impl Params, mut f: impl FnMut(&Row)) {
        match self.connection.prepare(sql) {
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
        match self.connection.execute(sql, params) {
            Ok(changed_rows) => changed_rows,
            Err(err) => panic!("{err}"),
        }
    }
}

pub trait DbItem {
    fn from(row: &Row) -> Self;
}

pub struct Card {
    pub id: Id,
    pub content: String,
    pub review: CardReview,
}

#[derive(Clone)]
pub struct CardReview {
    pub due_date: chrono::NaiveDate,
    pub due_days: usize,
    pub recall_attempts: usize,
    pub successful_recalls: usize,
}

impl DbItem for Card {
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

pub struct Tag {
    pub id: Id,
    pub name: String,
}

impl DbItem for Tag {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        }
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
