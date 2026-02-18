CREATE TABLE transactions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    counterparty_id INTEGER REFERENCES counterparties(id),
    category_id INTEGER REFERENCES categories(id),
    transaction_type_id INTEGER NOT NULL REFERENCES transaction_types(id),
    accounting_date TEXT NOT NULL,
    value_date TEXT NOT NULL,
    statement_number TEXT,
    transaction_number TEXT,
    amount_cents INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'EUR',
    description TEXT NOT NULL,
    communication TEXT,
    import_hash TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%S', 'now'))
);
