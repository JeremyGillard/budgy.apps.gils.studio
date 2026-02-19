# Budgy

Tauri 2 desktop app (Rust + vanilla JS) for tracking Belgian bank expenses, with a SQLite backend via Diesel ORM.

## Build & test commands

```bash
# Run all Rust tests (from repo root — Cargo workspace resolves src-tauri/)
cargo test -p budgy

# Dev mode (Vite dev server + Tauri window)
npm run tauri dev

# Production build
npm run tauri build

# Diesel migrations (run from src-tauri/)
diesel migration run
diesel migration generate <name>
```

## Architecture

Three-layer Rust backend inside `src-tauri/src/`:

```
commands/  →  services/  →  models/ + db/
                            csv/
```

- **commands/** — Thin `#[tauri::command]` wrappers. Lock the `DbConn` mutex, delegate to a service, return `Result<T, BudgyError>`. No business logic here.
- **services/** — Free functions taking `&mut SqliteConnection` as first arg. All business logic lives here.
- **models/** — Two-struct pattern per entity: `Entity` (Queryable/Selectable/Serialize) + `NewEntity<'a>` (Insertable, borrows strings).
- **db/** — `connection.rs` (establish_connection, run_migrations, establish_test_connection), `schema.rs` (Diesel auto-generated).
- **csv/** — `BankCsvParser` trait for bank-specific CSV import. Currently only `BelfiusParser`.

Shared state is `DbConn(Mutex<SqliteConnection>)`, defined in `commands/import_commands.rs`, registered via `app.manage()` in `lib.rs`.

Frontend is vanilla JS (`src/main.js`) — no framework. Uses `window.__TAURI__.core.invoke` for IPC.

## Key conventions

### Commit messages
Follow the [Conventional Commits](https://www.conventionalcommits.org/) convention (`feat:`, `fix:`, `refactor:`, `chore:`, `docs:`, etc.).

### TDD workflow
Write a failing test first, then implement. Tests live inline in each file under `#[cfg(test)] mod tests`.

### Test database
`establish_test_connection()` creates an in-memory SQLite (`:memory:`) with all migrations + seeds applied. No mocking — tests run real Diesel queries.

Some integration tests (e.g., CSV import) require a real Belfius CSV file in `.samples/` (gitignored, must exist locally).

### Amounts
Stored as **integer cents** (`i32`). Negative = expense, positive = income. Frontend divides by 100 for display.

### Dates
Stored as `TEXT` in `YYYY-MM-DD` format. `created_at`/`updated_at` use `strftime('%Y-%m-%dT%H:%M:%S', 'now')`. Date range queries use string comparison which works for ISO format.

### Import deduplication
SHA-256 hash of `account_iban + accounting_date + statement_number + transaction_number + amount_cents + description`. Stored in `import_hash TEXT NOT NULL UNIQUE`.

### CSV parsing
`BankCsvParser` trait with `detect(content: &[u8]) -> bool` and `parse(content: &[u8]) -> Result<Vec<CsvTransaction>, BudgyError>`. Belfius files use Windows-1252 encoding and semicolon delimiters.

### Error handling
Single `BudgyError` enum (`error.rs`) with `thiserror`. Variants: `Diesel`, `Migration`, `CsvParse`, `Io`, `General`. Implements `Serialize` (as string) for Tauri command returns. Diesel and IO errors auto-convert via `#[from]`.

### Services use `find_or_create` pattern
Accounts and counterparties use `find_or_create(conn, ...)` to deduplicate without erroring on existing rows.

## Key files

| File | Purpose |
|---|---|
| `src-tauri/src/lib.rs` | Tauri setup, state management, command registration |
| `src-tauri/src/error.rs` | `BudgyError` enum |
| `src-tauri/src/commands/import_commands.rs` | `DbConn` struct definition + `import_csv` command |
| `src-tauri/src/services/import_service.rs` | Core CSV import pipeline + `ImportResult` |
| `src-tauri/src/services/stats_service.rs` | `MonthlySummary` + `CategoryBreakdown` |
| `src-tauri/src/csv/parser_trait.rs` | `BankCsvParser` trait |
| `src-tauri/src/csv/belfius_parser.rs` | Belfius CSV parser implementation |
| `src-tauri/src/csv/types.rs` | `CsvTransaction` intermediate struct |
| `src-tauri/src/db/connection.rs` | DB connection helpers + `establish_test_connection` |
| `src-tauri/src/db/schema.rs` | Diesel auto-generated schema |
| `src-tauri/migrations/0007_seed_categories_and_types/` | Seeds 18 transaction types + 11 categories |
| `src/main.js` | All frontend logic (vanilla JS) |

### Completion workflow
When finishing work on a task:
1. Create a new branch from the current branch (e.g. `feat/short-description`)
2. Commit all changes with a conventional commit message
3. Push the branch and create a PR with the plan used as the PR description
4. Send a Slack notification (see below)

### Slack notifications
After completing a task, send a Slack notification via the webhook in `.env` (`SLACK_HOOK`). The message should be the PR title followed by the PR link (e.g. "feat: add category CRUD https://github.com/…/pull/1"). Use `curl` with the webhook URL.
