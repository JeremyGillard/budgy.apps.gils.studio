use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::counterparties;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = counterparties)]
pub struct Counterparty {
    pub id: i32,
    pub iban: Option<String>,
    pub name: String,
    pub street: Option<String>,
    pub postal_code_city: Option<String>,
    pub bic: Option<String>,
    pub country_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = counterparties)]
pub struct NewCounterparty<'a> {
    pub iban: Option<&'a str>,
    pub name: &'a str,
    pub street: Option<&'a str>,
    pub postal_code_city: Option<&'a str>,
    pub bic: Option<&'a str>,
    pub country_code: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    

    #[test]
    fn test_insert_and_query_counterparty() {
        let mut conn = establish_test_connection();

        let new = NewCounterparty {
            iban: Some("BE51 0834 4745 7262"),
            name: "Gillard Jeremy",
            street: None,
            postal_code_city: None,
            bic: Some("GKCCBEBB"),
            country_code: None,
        };

        diesel::insert_into(counterparties::table)
            .values(&new)
            .execute(&mut conn)
            .expect("Failed to insert counterparty");

        let results: Vec<Counterparty> = counterparties::table
            .select(Counterparty::as_select())
            .load(&mut conn)
            .expect("Failed to load counterparties");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Gillard Jeremy");
        assert_eq!(results[0].iban, Some("BE51 0834 4745 7262".to_string()));
    }

    #[test]
    fn test_iban_name_uniqueness() {
        let mut conn = establish_test_connection();

        let new = NewCounterparty {
            iban: Some("BE51 0834 4745 7262"),
            name: "Gillard Jeremy",
            street: None,
            postal_code_city: None,
            bic: None,
            country_code: None,
        };

        diesel::insert_into(counterparties::table)
            .values(&new)
            .execute(&mut conn)
            .expect("First insert should succeed");

        let result = diesel::insert_into(counterparties::table)
            .values(&new)
            .execute(&mut conn);

        assert!(result.is_err(), "Duplicate (iban, name) should fail");
    }

    #[test]
    fn test_nullable_iban() {
        let mut conn = establish_test_connection();

        let new = NewCounterparty {
            iban: None,
            name: "LSP*Oakberry Acai",
            street: None,
            postal_code_city: Some("1040 Bruxelles"),
            bic: None,
            country_code: Some("BE"),
        };

        diesel::insert_into(counterparties::table)
            .values(&new)
            .execute(&mut conn)
            .expect("Insert with null IBAN should succeed");

        let results: Vec<Counterparty> = counterparties::table
            .select(Counterparty::as_select())
            .load(&mut conn)
            .expect("Failed to load");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].iban, None);
    }
}
