use sha2::{Digest, Sha256};

use diesel::prelude::*;

use crate::csv::belfius_parser::BelfiusParser;
use crate::csv::parser_trait::BankCsvParser;
use crate::csv::types::CsvTransaction;
use crate::db::schema::{imports, transaction_types};
use crate::error::BudgyError;
use crate::models::import::NewImport;
use crate::models::transaction::NewTransaction;
use crate::services::{account_service, counterparty_service, transaction_service};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub total_parsed: usize,
    pub imported: usize,
    pub skipped_duplicates: usize,
    pub account_iban: String,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

fn compute_import_hash(tx: &CsvTransaction) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx.account_iban.as_bytes());
    hasher.update(tx.accounting_date.format("%Y-%m-%d").to_string().as_bytes());
    hasher.update(
        tx.statement_number
            .as_deref()
            .unwrap_or("")
            .as_bytes(),
    );
    hasher.update(
        tx.transaction_number
            .as_deref()
            .unwrap_or("")
            .as_bytes(),
    );
    hasher.update(tx.amount_cents.to_string().as_bytes());
    hasher.update(tx.description.as_bytes());
    hex::encode(hasher.finalize())
}

fn get_transaction_type_id(
    conn: &mut SqliteConnection,
    code: &str,
) -> Result<i32, BudgyError> {
    transaction_types::table
        .filter(transaction_types::code.eq(code))
        .select(transaction_types::id)
        .first::<i32>(conn)
        .optional()?
        .ok_or_else(|| {
            BudgyError::General(format!("Unknown transaction type code: {}", code))
        })
}

pub fn import_csv(
    conn: &mut SqliteConnection,
    filename: &str,
    content: &[u8],
) -> Result<ImportResult, BudgyError> {
    if !BelfiusParser::detect(content) {
        return Err(BudgyError::CsvParse(
            "Unrecognized bank CSV format".to_string(),
        ));
    }

    let parsed = BelfiusParser::parse(content)?;
    let total_parsed = parsed.len();

    if parsed.is_empty() {
        return Err(BudgyError::CsvParse("No transactions found in CSV".to_string()));
    }

    let account_iban = parsed[0].account_iban.clone();
    let currency = parsed[0].currency.clone();

    let account = account_service::find_or_create(conn, &account_iban, &currency)?;

    // Compute date range from ALL parsed transactions (not just imported ones)
    let date_from = parsed.iter()
        .map(|tx| tx.accounting_date.format("%Y-%m-%d").to_string())
        .min();
    let date_to = parsed.iter()
        .map(|tx| tx.accounting_date.format("%Y-%m-%d").to_string())
        .max();

    let mut imported = 0;
    let mut skipped = 0;

    for csv_tx in &parsed {
        let hash = compute_import_hash(csv_tx);

        if transaction_service::exists_by_hash(conn, &hash)? {
            skipped += 1;
            continue;
        }

        let type_id = get_transaction_type_id(conn, &csv_tx.transaction_type_code)?;

        let counterparty_id = if let Some(ref name) = csv_tx.counterparty_name {
            let c = counterparty_service::find_or_create(
                conn,
                csv_tx.counterparty_iban.as_deref(),
                name,
                csv_tx.counterparty_street.as_deref(),
                csv_tx.counterparty_postal_code_city.as_deref(),
                csv_tx.bic.as_deref(),
                csv_tx.country_code.as_deref(),
            )?;
            Some(c.id)
        } else {
            None
        };

        let acct_date = csv_tx.accounting_date.format("%Y-%m-%d").to_string();
        let val_date = csv_tx.value_date.format("%Y-%m-%d").to_string();

        let new = NewTransaction {
            account_id: account.id,
            counterparty_id,
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: &acct_date,
            value_date: &val_date,
            statement_number: csv_tx.statement_number.as_deref(),
            transaction_number: csv_tx.transaction_number.as_deref(),
            amount_cents: csv_tx.amount_cents,
            currency: &csv_tx.currency,
            description: &csv_tx.description,
            communication: csv_tx.communication.as_deref(),
            import_hash: &hash,
        };

        transaction_service::insert(conn, &new)?;
        imported += 1;
    }

    // Record the import
    let new_import = NewImport {
        filename,
        account_id: account.id,
        record_count: imported as i32,
        date_from: date_from.as_deref(),
        date_to: date_to.as_deref(),
    };

    diesel::insert_into(imports::table)
        .values(&new_import)
        .execute(conn)?;

    Ok(ImportResult {
        total_parsed,
        imported,
        skipped_duplicates: skipped,
        account_iban,
        date_from,
        date_to,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    use crate::db::schema::transactions;
    

    fn sample_csv() -> Vec<u8> {
        std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../.samples/BE34 0634 5590 5590 2025-08-08 9-59-57 1.csv"
        ))
        .expect("Sample CSV should exist")
    }

    #[test]
    fn test_compute_import_hash_deterministic() {
        let tx = CsvTransaction {
            account_iban: "BE34".to_string(),
            accounting_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            statement_number: Some("00001".to_string()),
            transaction_number: Some("39".to_string()),
            counterparty_iban: None,
            counterparty_name: None,
            counterparty_street: None,
            counterparty_postal_code_city: None,
            description: "Test".to_string(),
            value_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            amount_cents: -5000,
            currency: "EUR".to_string(),
            bic: None,
            country_code: None,
            communication: None,
            transaction_type_code: "ATM_WITHDRAWAL".to_string(),
        };

        let h1 = compute_import_hash(&tx);
        let h2 = compute_import_hash(&tx);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64); // SHA-256 hex
    }

    #[test]
    fn test_full_import_pipeline() {
        let conn = &mut establish_test_connection();
        let content = sample_csv();

        let result = import_csv(conn, "test.csv", &content).unwrap();

        assert_eq!(result.account_iban, "BE34 0634 5590 5590");
        assert!(result.imported >= 250, "Should import ~255 transactions, got {}", result.imported);
        assert_eq!(result.skipped_duplicates, 0);
        assert_eq!(result.total_parsed, result.imported);

        // Verify in DB
        let count: i64 = transactions::table
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(count as usize, result.imported);
    }

    #[test]
    fn test_reimport_dedup() {
        let conn = &mut establish_test_connection();
        let content = sample_csv();

        let first = import_csv(conn, "test.csv", &content).unwrap();
        assert!(first.imported > 0);

        let second = import_csv(conn, "test.csv", &content).unwrap();
        assert_eq!(second.imported, 0, "Re-import should skip all duplicates");
        assert_eq!(second.skipped_duplicates, first.total_parsed);
    }

    #[test]
    fn test_reimport_still_returns_date_range() {
        let conn = &mut establish_test_connection();
        let content = sample_csv();

        let first = import_csv(conn, "test.csv", &content).unwrap();
        assert!(first.date_from.is_some(), "First import should have date_from");
        assert!(first.date_to.is_some(), "First import should have date_to");

        let second = import_csv(conn, "re-import.csv", &content).unwrap();
        assert_eq!(second.imported, 0, "Re-import should skip all duplicates");
        assert_eq!(second.date_from, first.date_from, "Re-import should return same date_from");
        assert_eq!(second.date_to, first.date_to, "Re-import should return same date_to");
    }

    #[test]
    fn test_import_creates_account() {
        let conn = &mut establish_test_connection();
        let content = sample_csv();

        import_csv(conn, "test.csv", &content).unwrap();

        let accounts = account_service::list_all(conn).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].iban, "BE34 0634 5590 5590");
    }

    #[test]
    fn test_import_creates_counterparties() {
        let conn = &mut establish_test_connection();
        let content = sample_csv();

        import_csv(conn, "test.csv", &content).unwrap();

        let count: i64 = crate::db::schema::counterparties::table
            .count()
            .get_result(conn)
            .unwrap();
        assert!(count > 0, "Should have created counterparties");
    }
}
