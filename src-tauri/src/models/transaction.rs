use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::transactions;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub account_id: i32,
    pub counterparty_id: Option<i32>,
    pub category_id: Option<i32>,
    pub transaction_type_id: i32,
    pub accounting_date: String,
    pub value_date: String,
    pub statement_number: Option<String>,
    pub transaction_number: Option<String>,
    pub amount_cents: i32,
    pub currency: String,
    pub description: String,
    pub communication: Option<String>,
    pub import_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub account_id: i32,
    pub counterparty_id: Option<i32>,
    pub category_id: Option<i32>,
    pub transaction_type_id: i32,
    pub accounting_date: &'a str,
    pub value_date: &'a str,
    pub statement_number: Option<&'a str>,
    pub transaction_number: Option<&'a str>,
    pub amount_cents: i32,
    pub currency: &'a str,
    pub description: &'a str,
    pub communication: Option<&'a str>,
    pub import_hash: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    use crate::db::schema::{accounts, transaction_types};
    use crate::models::account::NewAccount;
    

    fn setup_account(conn: &mut diesel::SqliteConnection) -> i32 {
        let new = NewAccount {
            iban: "BE34 0634 5590 5590",
            label: None,
            currency: "EUR",
        };
        diesel::insert_into(accounts::table)
            .values(&new)
            .execute(conn)
            .unwrap();
        accounts::table
            .select(accounts::id)
            .first::<i32>(conn)
            .unwrap()
    }

    fn get_transaction_type_id(conn: &mut diesel::SqliteConnection, code: &str) -> i32 {
        transaction_types::table
            .filter(transaction_types::code.eq(code))
            .select(transaction_types::id)
            .first::<i32>(conn)
            .unwrap()
    }

    #[test]
    fn test_insert_and_query_transaction() {
        let conn = &mut establish_test_connection();
        let account_id = setup_account(conn);
        let type_id = get_transaction_type_id(conn, "ATM_WITHDRAWAL");

        let new = NewTransaction {
            account_id,
            counterparty_id: None,
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-31",
            value_date: "2024-12-31",
            statement_number: Some("00001"),
            transaction_number: Some("39"),
            amount_cents: -5000,
            currency: "EUR",
            description: "RETRAIT D'ESPECES",
            communication: None,
            import_hash: "abc123",
        };

        diesel::insert_into(transactions::table)
            .values(&new)
            .execute(conn)
            .expect("Failed to insert transaction");

        let results: Vec<Transaction> = transactions::table
            .select(Transaction::as_select())
            .load(conn)
            .expect("Failed to load transactions");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].amount_cents, -5000);
        assert_eq!(results[0].account_id, account_id);
    }

    #[test]
    fn test_import_hash_uniqueness() {
        let conn = &mut establish_test_connection();
        let account_id = setup_account(conn);
        let type_id = get_transaction_type_id(conn, "ATM_WITHDRAWAL");

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
            import_hash: "unique_hash_1",
        };

        diesel::insert_into(transactions::table)
            .values(&new)
            .execute(conn)
            .expect("First insert should succeed");

        let dup = NewTransaction {
            import_hash: "unique_hash_1",
            ..new
        };

        let result = diesel::insert_into(transactions::table)
            .values(&dup)
            .execute(conn);

        assert!(result.is_err(), "Duplicate import_hash should fail");
    }
}
