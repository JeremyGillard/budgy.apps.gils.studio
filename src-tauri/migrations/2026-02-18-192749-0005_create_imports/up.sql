CREATE TABLE imports (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    filename TEXT NOT NULL,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    imported_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    record_count INTEGER NOT NULL,
    date_from TEXT,
    date_to TEXT
);
