use diesel::prelude::*;
use serde::Serialize;

use crate::db::schema::transactions;
use crate::error::BudgyError;
use crate::models::transaction::{NewTransaction, Transaction};

#[derive(Serialize)]
pub struct CategorizationStats {
    pub total: i64,
    pub uncategorized: i64,
}

pub fn insert(
    conn: &mut SqliteConnection,
    new: &NewTransaction,
) -> Result<(), BudgyError> {
    diesel::insert_into(transactions::table)
        .values(new)
        .execute(conn)?;
    Ok(())
}

pub fn exists_by_hash(conn: &mut SqliteConnection, hash: &str) -> Result<bool, BudgyError> {
    let count: i64 = transactions::table
        .filter(transactions::import_hash.eq(hash))
        .count()
        .get_result(conn)?;
    Ok(count > 0)
}

pub fn list_by_month(
    conn: &mut SqliteConnection,
    year: i32,
    month: u32,
) -> Result<Vec<Transaction>, BudgyError> {
    let start = format!("{:04}-{:02}-01", year, month);
    let end = format!("{:04}-{:02}-31", year, month);

    transactions::table
        .filter(transactions::accounting_date.ge(&start))
        .filter(transactions::accounting_date.le(&end))
        .order(transactions::accounting_date.desc())
        .select(Transaction::as_select())
        .load(conn)
        .map_err(BudgyError::from)
}

pub fn list_by_account(
    conn: &mut SqliteConnection,
    account_id: i32,
) -> Result<Vec<Transaction>, BudgyError> {
    transactions::table
        .filter(transactions::account_id.eq(account_id))
        .order(transactions::accounting_date.desc())
        .select(Transaction::as_select())
        .load(conn)
        .map_err(BudgyError::from)
}

pub fn categorize(
    conn: &mut SqliteConnection,
    transaction_id: i32,
    category_id: i32,
) -> Result<(), BudgyError> {
    diesel::update(transactions::table.filter(transactions::id.eq(transaction_id)))
        .set(transactions::category_id.eq(category_id))
        .execute(conn)?;
    Ok(())
}

pub fn categorization_stats(
    conn: &mut SqliteConnection,
) -> Result<CategorizationStats, BudgyError> {
    let total: i64 = transactions::table.count().get_result(conn)?;
    let uncategorized: i64 = transactions::table
        .filter(transactions::category_id.is_null())
        .count()
        .get_result(conn)?;
    Ok(CategorizationStats {
        total,
        uncategorized,
    })
}

pub fn bulk_categorize(
    conn: &mut SqliteConnection,
    transaction_ids: &[i32],
    category_id: i32,
) -> Result<usize, BudgyError> {
    if transaction_ids.is_empty() {
        return Ok(0);
    }
    let updated = diesel::update(
        transactions::table.filter(transactions::id.eq_any(transaction_ids)),
    )
    .set(transactions::category_id.eq(category_id))
    .execute(conn)?;
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    use crate::db::schema::{accounts, categories, transaction_types};
    use crate::models::account::NewAccount;
    use crate::models::category::Category;

    fn setup(conn: &mut SqliteConnection) -> (i32, i32) {
        let acct = NewAccount {
            iban: "BE34 0634 5590 5590",
            label: None,
            currency: "EUR",
        };
        diesel::insert_into(accounts::table)
            .values(&acct)
            .execute(conn)
            .unwrap();
        let account_id: i32 = accounts::table.select(accounts::id).first(conn).unwrap();

        let type_id: i32 = transaction_types::table
            .filter(transaction_types::code.eq("ATM_WITHDRAWAL"))
            .select(transaction_types::id)
            .first(conn)
            .unwrap();

        (account_id, type_id)
    }

    #[test]
    fn test_insert_and_exists() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        let new = NewTransaction {
            account_id,
            counterparty_id: None,
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-31",
            value_date: "2024-12-31",
            statement_number: None,
            transaction_number: None,
            amount_cents: -5000,
            currency: "EUR",
            description: "Test",
            communication: None,
            import_hash: "hash123",
        };

        assert!(!exists_by_hash(conn, "hash123").unwrap());
        insert(conn, &new).unwrap();
        assert!(exists_by_hash(conn, "hash123").unwrap());
    }

    #[test]
    fn test_list_by_month() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        for (i, date) in ["2024-12-15", "2024-12-20", "2024-11-05"].iter().enumerate() {
            let new = NewTransaction {
                account_id,
                counterparty_id: None,
                category_id: None,
                transaction_type_id: type_id,
                accounting_date: date,
                value_date: date,
                statement_number: None,
                transaction_number: None,
                amount_cents: -1000,
                currency: "EUR",
                description: "Test",
                communication: None,
                import_hash: &format!("hash_{}", i),
            };
            insert(conn, &new).unwrap();
        }

        let dec = list_by_month(conn, 2024, 12).unwrap();
        assert_eq!(dec.len(), 2);

        let nov = list_by_month(conn, 2024, 11).unwrap();
        assert_eq!(nov.len(), 1);
    }

    #[test]
    fn test_bulk_categorize() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        // Insert 3 transactions
        for i in 0..3 {
            let new = NewTransaction {
                account_id,
                counterparty_id: None,
                category_id: None,
                transaction_type_id: type_id,
                accounting_date: "2024-12-15",
                value_date: "2024-12-15",
                statement_number: None,
                transaction_number: None,
                amount_cents: -1000 * (i + 1),
                currency: "EUR",
                description: "Test",
                communication: None,
                import_hash: &format!("bulk_hash_{}", i),
            };
            insert(conn, &new).unwrap();
        }

        let all: Vec<Transaction> = transactions::table
            .order(transactions::id.asc())
            .select(Transaction::as_select())
            .load(conn)
            .unwrap();
        assert_eq!(all.len(), 3);

        let food: Category = categories::table
            .filter(categories::name.eq("Food & Groceries"))
            .first(conn)
            .unwrap();

        // Bulk categorize first 2
        let ids = vec![all[0].id, all[1].id];
        let count = bulk_categorize(conn, &ids, food.id).unwrap();
        assert_eq!(count, 2);

        // Verify first 2 are categorized
        let t1: Transaction = transactions::table.find(all[0].id).first(conn).unwrap();
        assert_eq!(t1.category_id, Some(food.id));
        let t2: Transaction = transactions::table.find(all[1].id).first(conn).unwrap();
        assert_eq!(t2.category_id, Some(food.id));

        // Verify 3rd is unchanged
        let t3: Transaction = transactions::table.find(all[2].id).first(conn).unwrap();
        assert_eq!(t3.category_id, None);
    }

    #[test]
    fn test_bulk_categorize_empty_list() {
        let conn = &mut establish_test_connection();
        let count = bulk_categorize(conn, &[], 1).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_categorization_stats() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        // Insert 3 transactions, all uncategorized
        for i in 0..3 {
            let new = NewTransaction {
                account_id,
                counterparty_id: None,
                category_id: None,
                transaction_type_id: type_id,
                accounting_date: "2024-12-15",
                value_date: "2024-12-15",
                statement_number: None,
                transaction_number: None,
                amount_cents: -1000 * (i + 1),
                currency: "EUR",
                description: "Test",
                communication: None,
                import_hash: &format!("stats_hash_{}", i),
            };
            insert(conn, &new).unwrap();
        }

        // Categorize one of them
        let food: Category = categories::table
            .filter(categories::name.eq("Food & Groceries"))
            .first(conn)
            .unwrap();
        let first_tx: Transaction = transactions::table
            .order(transactions::id.asc())
            .first(conn)
            .unwrap();
        categorize(conn, first_tx.id, food.id).unwrap();

        let stats = categorization_stats(conn).unwrap();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.uncategorized, 2);
    }

    #[test]
    fn test_categorize() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        let new = NewTransaction {
            account_id,
            counterparty_id: None,
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-31",
            value_date: "2024-12-31",
            statement_number: None,
            transaction_number: None,
            amount_cents: -5000,
            currency: "EUR",
            description: "Test",
            communication: None,
            import_hash: "cat_hash",
        };
        insert(conn, &new).unwrap();

        let tx: Transaction = transactions::table.first(conn).unwrap();
        assert_eq!(tx.category_id, None);

        let food: Category = categories::table
            .filter(categories::name.eq("Food & Groceries"))
            .first(conn)
            .unwrap();

        categorize(conn, tx.id, food.id).unwrap();

        let tx: Transaction = transactions::table.first(conn).unwrap();
        assert_eq!(tx.category_id, Some(food.id));
    }
}
