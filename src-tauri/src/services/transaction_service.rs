use diesel::prelude::*;

use crate::db::schema::transactions;
use crate::error::BudgyError;
use crate::models::transaction::{NewTransaction, Transaction};

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
