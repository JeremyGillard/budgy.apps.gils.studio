use diesel::prelude::*;

use crate::db::schema::counterparties;
use crate::error::BudgyError;
use crate::models::counterparty::{Counterparty, NewCounterparty};

pub fn find_or_create(
    conn: &mut SqliteConnection,
    iban: Option<&str>,
    name: &str,
    street: Option<&str>,
    postal_code_city: Option<&str>,
    bic: Option<&str>,
    country_code: Option<&str>,
) -> Result<Counterparty, BudgyError> {
    let existing = match iban {
        Some(iban_val) => counterparties::table
            .filter(
                counterparties::iban
                    .eq(iban_val)
                    .and(counterparties::name.eq(name)),
            )
            .first::<Counterparty>(conn)
            .optional()?,
        None => counterparties::table
            .filter(
                counterparties::iban
                    .is_null()
                    .and(counterparties::name.eq(name)),
            )
            .first::<Counterparty>(conn)
            .optional()?,
    };

    if let Some(c) = existing {
        return Ok(c);
    }

    let new = NewCounterparty {
        iban,
        name,
        street,
        postal_code_city,
        bic,
        country_code,
    };

    diesel::insert_into(counterparties::table)
        .values(&new)
        .execute(conn)?;

    // Re-fetch
    let result = match iban {
        Some(iban_val) => counterparties::table
            .filter(
                counterparties::iban
                    .eq(iban_val)
                    .and(counterparties::name.eq(name)),
            )
            .first::<Counterparty>(conn)?,
        None => counterparties::table
            .filter(
                counterparties::iban
                    .is_null()
                    .and(counterparties::name.eq(name)),
            )
            .first::<Counterparty>(conn)?,
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;

    #[test]
    fn test_find_or_create_with_iban() {
        let conn = &mut establish_test_connection();
        let c = find_or_create(
            conn,
            Some("BE51 0834 4745 7262"),
            "Gillard Jeremy",
            None,
            None,
            Some("GKCCBEBB"),
            None,
        )
        .unwrap();
        assert_eq!(c.name, "Gillard Jeremy");
        assert_eq!(c.iban, Some("BE51 0834 4745 7262".to_string()));
    }

    #[test]
    fn test_find_or_create_without_iban() {
        let conn = &mut establish_test_connection();
        let c = find_or_create(
            conn,
            None,
            "LSP*Oakberry Acai",
            None,
            Some("1040 Bruxelles"),
            None,
            Some("BE"),
        )
        .unwrap();
        assert_eq!(c.iban, None);
        assert_eq!(c.name, "LSP*Oakberry Acai");
    }

    #[test]
    fn test_find_or_create_dedup() {
        let conn = &mut establish_test_connection();
        let c1 = find_or_create(conn, Some("BE51"), "Test", None, None, None, None).unwrap();
        let c2 = find_or_create(conn, Some("BE51"), "Test", None, None, None, None).unwrap();
        assert_eq!(c1.id, c2.id);
    }
}
