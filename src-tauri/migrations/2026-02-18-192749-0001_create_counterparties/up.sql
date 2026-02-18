CREATE TABLE counterparties (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    iban TEXT,
    name TEXT NOT NULL,
    street TEXT,
    postal_code_city TEXT,
    bic TEXT,
    country_code TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    UNIQUE(iban, name)
);
