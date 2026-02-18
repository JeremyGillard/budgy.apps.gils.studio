use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::imports;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = imports)]
pub struct Import {
    pub id: i32,
    pub filename: String,
    pub account_id: i32,
    pub imported_at: String,
    pub record_count: i32,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = imports)]
pub struct NewImport<'a> {
    pub filename: &'a str,
    pub account_id: i32,
    pub record_count: i32,
    pub date_from: Option<&'a str>,
    pub date_to: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    use crate::db::schema::accounts;
    use crate::models::account::NewAccount;
    

    #[test]
    fn test_insert_and_query_import() {
        let conn = &mut establish_test_connection();

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

        let new = NewImport {
            filename: "test.csv",
            account_id,
            record_count: 255,
            date_from: Some("2024-08-01"),
            date_to: Some("2024-12-31"),
        };

        diesel::insert_into(imports::table)
            .values(&new)
            .execute(conn)
            .expect("Failed to insert import");

        let results: Vec<Import> = imports::table
            .select(Import::as_select())
            .load(conn)
            .expect("Failed to load imports");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].record_count, 255);
        assert_eq!(results[0].filename, "test.csv");
    }
}
