use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::*;
use crate::services::csv_parser;

pub struct ImportFileResult {
    pub file_name: String,
    pub imported_count: usize,
    pub skipped_count: usize,
}

pub fn import_file(
    conn: &mut SqliteConnection,
    file_path: &str,
) -> Result<ImportFileResult, String> {
    let raw_content = std::fs::read(file_path).map_err(|e| format!("Cannot read file: {}", e))?;
    let file_hash = csv_parser::compute_sha256(&raw_content);

    // Check for duplicate file
    let existing: i64 = imports::table
        .filter(imports::file_hash.eq(&file_hash))
        .count()
        .get_result(conn)
        .map_err(|e| e.to_string())?;

    if existing > 0 {
        return Err("File already imported (duplicate hash)".to_string());
    }

    let parsed = csv_parser::parse_csv(&raw_content)?;

    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(file_path)
        .to_string();

    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    // Find or create account
    let account_id = find_or_create_account(conn, &parsed.account_iban, &now)?;

    // Compute date range
    let date_range_from = parsed
        .transactions
        .iter()
        .map(|t| t.accounting_date.as_str())
        .min()
        .map(|s| s.to_string());
    let date_range_to = parsed
        .transactions
        .iter()
        .map(|t| t.accounting_date.as_str())
        .max()
        .map(|s| s.to_string());

    // Insert import record
    let new_import = NewImport {
        account_id,
        file_name: &file_name,
        file_hash: &file_hash,
        record_count: parsed.transactions.len() as i32,
        date_range_from: date_range_from.as_deref(),
        date_range_to: date_range_to.as_deref(),
        imported_at: &now,
    };

    diesel::insert_into(imports::table)
        .values(&new_import)
        .execute(conn)
        .map_err(|e| e.to_string())?;

    let import_id: i32 = imports::table
        .order(imports::id.desc())
        .select(imports::id)
        .first(conn)
        .map_err(|e| e.to_string())?;

    // Insert metadata
    for meta in &parsed.metadata {
        let new_meta = NewImportMetadata {
            import_id,
            key: &meta.key,
            value: &meta.value,
        };
        diesel::insert_into(import_metadata::table)
            .values(&new_meta)
            .execute(conn)
            .map_err(|e| e.to_string())?;
    }

    // Insert transactions
    let mut imported_count = 0;
    let mut skipped_count = 0;

    for txn in &parsed.transactions {
        // Check for row-level dedup
        let row_exists: i64 = transactions::table
            .filter(transactions::row_hash.eq(&txn.row_hash))
            .count()
            .get_result(conn)
            .map_err(|e| e.to_string())?;

        if row_exists > 0 {
            skipped_count += 1;
            continue;
        }

        // Resolve transaction type
        let transaction_type_id =
            resolve_transaction_type(conn, &txn.transaction_description)?;

        let new_txn = NewTransaction {
            import_id,
            account_id,
            transaction_type_id,
            accounting_date: &txn.accounting_date,
            statement_number: &txn.statement_number,
            sequence_number: txn.sequence_number,
            counterparty_account: txn.counterparty_account.as_deref(),
            counterparty_name: txn.counterparty_name.as_deref(),
            counterparty_street: txn.counterparty_street.as_deref(),
            counterparty_city: txn.counterparty_city.as_deref(),
            transaction_description: &txn.transaction_description,
            value_date: &txn.value_date,
            amount_cents: txn.amount_cents,
            currency: &txn.currency,
            bic: txn.bic.as_deref(),
            country_code: txn.country_code.as_deref(),
            communication: txn.communication.as_deref(),
            row_hash: &txn.row_hash,
            created_at: &now,
            updated_at: &now,
        };

        diesel::insert_into(transactions::table)
            .values(&new_txn)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        imported_count += 1;
    }

    Ok(ImportFileResult {
        file_name,
        imported_count,
        skipped_count,
    })
}

fn find_or_create_account(
    conn: &mut SqliteConnection,
    iban: &str,
    now: &str,
) -> Result<i32, String> {
    let existing = accounts::table
        .filter(accounts::iban.eq(iban))
        .select(accounts::id)
        .first::<i32>(conn)
        .optional()
        .map_err(|e| e.to_string())?;

    if let Some(id) = existing {
        return Ok(id);
    }

    let new_account = NewAccount {
        iban,
        name: None,
        currency: "EUR",
        created_at: now,
        updated_at: now,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .execute(conn)
        .map_err(|e| e.to_string())?;

    accounts::table
        .order(accounts::id.desc())
        .select(accounts::id)
        .first::<i32>(conn)
        .map_err(|e| e.to_string())
}

fn resolve_transaction_type(
    conn: &mut SqliteConnection,
    description: &str,
) -> Result<Option<i32>, String> {
    let code = match csv_parser::match_transaction_type(description) {
        Some(c) => c,
        None => return Ok(None),
    };

    let type_id = transaction_types::table
        .filter(transaction_types::code.eq(code))
        .select(transaction_types::id)
        .first::<i32>(conn)
        .optional()
        .map_err(|e| e.to_string())?;

    Ok(type_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use diesel_migrations::MigrationHarness;

    fn setup_conn() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:")
            .expect("Failed to create in-memory database");
        conn.run_pending_migrations(db::MIGRATIONS)
            .expect("Failed to run migrations");
        conn
    }

    #[test]
    fn test_find_or_create_account_creates_new() {
        let mut conn = setup_conn();
        let now = "2024-01-01T00:00:00";
        let id = find_or_create_account(&mut conn, "BE34 0634 5590 5590", now).unwrap();
        assert!(id > 0);

        // Second call should return same id
        let id2 = find_or_create_account(&mut conn, "BE34 0634 5590 5590", now).unwrap();
        assert_eq!(id, id2);
    }

    #[test]
    fn test_find_or_create_account_different_ibans() {
        let mut conn = setup_conn();
        let now = "2024-01-01T00:00:00";
        let id1 = find_or_create_account(&mut conn, "BE34 0634 5590 5590", now).unwrap();
        let id2 = find_or_create_account(&mut conn, "BE51 0834 4745 7262", now).unwrap();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_resolve_transaction_type_known() {
        let mut conn = setup_conn();
        let result =
            resolve_transaction_type(&mut conn, "BANCONTACT ACHAT - COLRUYT").unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_resolve_transaction_type_unknown() {
        let mut conn = setup_conn();
        let result =
            resolve_transaction_type(&mut conn, "SOMETHING UNKNOWN").unwrap();
        assert!(result.is_none());
    }
}
