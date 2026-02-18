CREATE TABLE category_rules (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    category_id INTEGER NOT NULL REFERENCES categories(id),
    match_field TEXT NOT NULL,
    match_pattern TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0
);
