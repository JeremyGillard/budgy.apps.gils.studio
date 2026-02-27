CREATE TABLE transactions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    import_id INTEGER NOT NULL REFERENCES imports(id),
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    transaction_type_id INTEGER REFERENCES transaction_types(id),
    accounting_date TEXT NOT NULL,
    statement_number TEXT NOT NULL,
    sequence_number INTEGER NOT NULL,
    counterparty_account TEXT,
    counterparty_name TEXT,
    counterparty_street TEXT,
    counterparty_city TEXT,
    transaction_description TEXT NOT NULL,
    value_date TEXT NOT NULL,
    amount_cents INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'EUR',
    bic TEXT,
    country_code TEXT,
    communication TEXT,
    row_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (account_id, accounting_date, statement_number, sequence_number)
);

CREATE INDEX idx_transactions_accounting_date ON transactions(accounting_date);
CREATE INDEX idx_transactions_value_date ON transactions(value_date);
CREATE INDEX idx_transactions_account_id ON transactions(account_id);
CREATE INDEX idx_transactions_amount_cents ON transactions(amount_cents);
CREATE INDEX idx_transactions_counterparty_name ON transactions(counterparty_name);
CREATE INDEX idx_transactions_row_hash ON transactions(row_hash);
CREATE INDEX idx_transactions_transaction_type_id ON transactions(transaction_type_id);
