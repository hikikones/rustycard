CREATE TABLE cards (
    card_id INTEGER PRIMARY KEY,
    content TEXT NOT NULL,
    due_date DATE DEFAULT (date('now', '+1 day')) NOT NULL,
    due_days INT DEFAULT 1 NOT NULL,
    attempts INT DEFAULT 0 NOT NULL,
    successes INT DEFAULT 0 NOT NULL
    -- FOREIGN KEY (topic_id) REFERENCES topics (topic_id)
    --     ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE tags (
    tag_id INTEGER PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE card_tag (
    card_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (card_id, tag_id)
    -- TODO: FOREIGN KEY CASCADE
);

INSERT INTO tags (name)
    VALUES  ("Tag1"),
            ("Tag2"),
            ("Tag3"),
            ("Tag4");

INSERT INTO cards (content)
    VALUES  ("**content**\n\n---\n\n__Back__"),
            ("ok\n\n---\n\nlol"),
            ("yes\n\n---\n\nno");

INSERT INTO cards (content, due_date)
    VALUES  ("TODAY\n\n---\n\nCard is scheduled for today", date('now')),
            ("TOMORROW\n\n---\n\nCard is scheduled for tomorrow", date('now', '+1 day'));

INSERT INTO card_tag (card_id, tag_id)
    VALUES  (1, 1),
            (2, 1),
            (2, 2),
            (2, 3),
            (5, 2),
            (5, 3);
