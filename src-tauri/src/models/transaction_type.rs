use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::transaction_types;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = transaction_types)]
pub struct TransactionType {
    pub id: i32,
    pub code: String,
    pub label: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = transaction_types)]
pub struct NewTransactionType<'a> {
    pub code: &'a str,
    pub label: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    

    #[test]
    fn test_seed_transaction_types_exist() {
        let conn = &mut establish_test_connection();

        let results: Vec<TransactionType> = transaction_types::table
            .select(TransactionType::as_select())
            .load(conn)
            .expect("Failed to load transaction types");

        assert!(results.len() >= 18, "Seed transaction types should be present");

        let codes: Vec<&str> = results.iter().map(|t| t.code.as_str()).collect();
        assert!(codes.contains(&"BANCONTACT_PURCHASE"));
        assert!(codes.contains(&"TRANSFER_OUT"));
        assert!(codes.contains(&"DIRECT_DEBIT"));
        assert!(codes.contains(&"OTHER"));
    }

    #[test]
    fn test_code_uniqueness() {
        let conn = &mut establish_test_connection();

        let dup = NewTransactionType {
            code: "ATM_WITHDRAWAL",
            label: "Duplicate",
        };

        let result = diesel::insert_into(transaction_types::table)
            .values(&dup)
            .execute(conn);

        assert!(result.is_err(), "Duplicate code should fail");
    }
}
