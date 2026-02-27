CREATE TABLE imports (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    file_name TEXT NOT NULL,
    file_hash TEXT NOT NULL UNIQUE,
    record_count INTEGER NOT NULL,
    date_range_from TEXT,
    date_range_to TEXT,
    imported_at TEXT NOT NULL
);
