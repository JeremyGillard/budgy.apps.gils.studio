use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::accounts;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub iban: String,
    pub label: Option<String>,
    pub currency: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub iban: &'a str,
    pub label: Option<&'a str>,
    pub currency: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    

    #[test]
    fn test_insert_and_query_account() {
        let mut conn = establish_test_connection();

        let new = NewAccount {
            iban: "BE34 0634 5590 5590",
            label: Some("Main Account"),
            currency: "EUR",
        };

        diesel::insert_into(accounts::table)
            .values(&new)
            .execute(&mut conn)
            .expect("Failed to insert account");

        let results: Vec<Account> = accounts::table
            .select(Account::as_select())
            .load(&mut conn)
            .expect("Failed to load accounts");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].iban, "BE34 0634 5590 5590");
        assert_eq!(results[0].label, Some("Main Account".to_string()));
        assert_eq!(results[0].currency, "EUR");
    }

    #[test]
    fn test_iban_uniqueness() {
        let mut conn = establish_test_connection();

        let new = NewAccount {
            iban: "BE34 0634 5590 5590",
            label: None,
            currency: "EUR",
        };

        diesel::insert_into(accounts::table)
            .values(&new)
            .execute(&mut conn)
            .expect("First insert should succeed");

        let result = diesel::insert_into(accounts::table)
            .values(&new)
            .execute(&mut conn);

        assert!(result.is_err(), "Duplicate IBAN should fail");
    }
}
