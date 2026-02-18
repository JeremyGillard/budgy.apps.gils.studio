use diesel::prelude::*;

use crate::db::schema::accounts;
use crate::error::BudgyError;
use crate::models::account::{Account, NewAccount};

pub fn find_or_create(
    conn: &mut SqliteConnection,
    iban: &str,
    currency: &str,
) -> Result<Account, BudgyError> {
    if let Some(existing) = accounts::table
        .filter(accounts::iban.eq(iban))
        .first::<Account>(conn)
        .optional()?
    {
        return Ok(existing);
    }

    let new = NewAccount {
        iban,
        label: None,
        currency,
    };

    diesel::insert_into(accounts::table)
        .values(&new)
        .execute(conn)?;

    accounts::table
        .filter(accounts::iban.eq(iban))
        .first::<Account>(conn)
        .map_err(BudgyError::from)
}

pub fn list_all(conn: &mut SqliteConnection) -> Result<Vec<Account>, BudgyError> {
    accounts::table
        .select(Account::as_select())
        .load(conn)
        .map_err(BudgyError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;

    #[test]
    fn test_find_or_create_new() {
        let conn = &mut establish_test_connection();
        let account = find_or_create(conn, "BE34 0634 5590 5590", "EUR").unwrap();
        assert_eq!(account.iban, "BE34 0634 5590 5590");
        assert_eq!(account.currency, "EUR");
    }

    #[test]
    fn test_find_or_create_existing() {
        let conn = &mut establish_test_connection();
        let a1 = find_or_create(conn, "BE34 0634 5590 5590", "EUR").unwrap();
        let a2 = find_or_create(conn, "BE34 0634 5590 5590", "EUR").unwrap();
        assert_eq!(a1.id, a2.id);
    }

    #[test]
    fn test_list_all() {
        let conn = &mut establish_test_connection();
        find_or_create(conn, "BE34 0634 5590 5590", "EUR").unwrap();
        find_or_create(conn, "BE51 0834 4745 7262", "EUR").unwrap();
        let all = list_all(conn).unwrap();
        assert_eq!(all.len(), 2);
    }
}
