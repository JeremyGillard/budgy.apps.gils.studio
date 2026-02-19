use diesel::prelude::*;
use diesel::sql_types::{Integer, Nullable, Text};
use serde::Serialize;

use crate::db::schema::transactions;
use crate::error::BudgyError;
use crate::models::transaction::NewTransaction;

#[derive(Debug, Serialize, QueryableByName)]
pub struct CategorySuggestion {
    #[diesel(sql_type = Integer)]
    pub transaction_id: i32,
    #[diesel(sql_type = Integer)]
    pub suggested_category_id: i32,
}

#[derive(Serialize)]
pub struct CategorizationStats {
    pub total: i64,
    pub uncategorized: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct TransactionWithCounterparty {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub account_id: i32,
    #[diesel(sql_type = Nullable<Integer>)]
    pub counterparty_id: Option<i32>,
    #[diesel(sql_type = Nullable<Integer>)]
    pub category_id: Option<i32>,
    #[diesel(sql_type = Integer)]
    pub transaction_type_id: i32,
    #[diesel(sql_type = Text)]
    pub accounting_date: String,
    #[diesel(sql_type = Text)]
    pub value_date: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub statement_number: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub transaction_number: Option<String>,
    #[diesel(sql_type = Integer)]
    pub amount_cents: i32,
    #[diesel(sql_type = Text)]
    pub currency: String,
    #[diesel(sql_type = Text)]
    pub description: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub communication: Option<String>,
    #[diesel(sql_type = Text)]
    pub import_hash: String,
    #[diesel(sql_type = Text)]
    pub created_at: String,
    #[diesel(sql_type = Text)]
    pub updated_at: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub counterparty_name: Option<String>,
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
) -> Result<Vec<TransactionWithCounterparty>, BudgyError> {
    let start = format!("{:04}-{:02}-01", year, month);
    let end = format!("{:04}-{:02}-31", year, month);

    diesel::sql_query(
        "SELECT t.id, t.account_id, t.counterparty_id, t.category_id, \
         t.transaction_type_id, t.accounting_date, t.value_date, \
         t.statement_number, t.transaction_number, t.amount_cents, \
         t.currency, t.description, t.communication, t.import_hash, \
         t.created_at, t.updated_at, \
         cp.name AS counterparty_name \
         FROM transactions t \
         LEFT JOIN counterparties cp ON t.counterparty_id = cp.id \
         WHERE t.accounting_date >= ?1 AND t.accounting_date <= ?2 \
         ORDER BY t.accounting_date DESC",
    )
    .bind::<Text, _>(&start)
    .bind::<Text, _>(&end)
    .load::<TransactionWithCounterparty>(conn)
    .map_err(BudgyError::from)
}

pub fn list_by_account(
    conn: &mut SqliteConnection,
    account_id: i32,
) -> Result<Vec<TransactionWithCounterparty>, BudgyError> {
    diesel::sql_query(
        "SELECT t.id, t.account_id, t.counterparty_id, t.category_id, \
         t.transaction_type_id, t.accounting_date, t.value_date, \
         t.statement_number, t.transaction_number, t.amount_cents, \
         t.currency, t.description, t.communication, t.import_hash, \
         t.created_at, t.updated_at, \
         cp.name AS counterparty_name \
         FROM transactions t \
         LEFT JOIN counterparties cp ON t.counterparty_id = cp.id \
         WHERE t.account_id = ?1 \
         ORDER BY t.accounting_date DESC",
    )
    .bind::<Integer, _>(account_id)
    .load::<TransactionWithCounterparty>(conn)
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

pub fn suggest_categories(
    conn: &mut SqliteConnection,
    year: i32,
    month: u32,
) -> Result<Vec<CategorySuggestion>, BudgyError> {
    let start = format!("{:04}-{:02}-01", year, month);
    let end = format!("{:04}-{:02}-31", year, month);

    let results = diesel::sql_query(
        "WITH uncategorized AS ( \
           SELECT id, counterparty_id, description \
           FROM transactions \
           WHERE category_id IS NULL \
             AND accounting_date >= ?1 AND accounting_date <= ?2 \
         ), \
         cp_suggestions AS ( \
           SELECT u.id AS transaction_id, t.category_id AS suggested_category_id, COUNT(*) AS freq \
           FROM uncategorized u \
           JOIN transactions t ON t.counterparty_id = u.counterparty_id \
           WHERE t.category_id IS NOT NULL AND u.counterparty_id IS NOT NULL \
           GROUP BY u.id, t.category_id \
         ), \
         best_cp AS ( \
           SELECT transaction_id, suggested_category_id \
           FROM cp_suggestions cs \
           WHERE freq = (SELECT MAX(freq) FROM cp_suggestions WHERE transaction_id = cs.transaction_id) \
           GROUP BY transaction_id \
         ), \
         desc_suggestions AS ( \
           SELECT u.id AS transaction_id, t.category_id AS suggested_category_id, COUNT(*) AS freq \
           FROM uncategorized u \
           LEFT JOIN best_cp bc ON bc.transaction_id = u.id \
           JOIN transactions t ON t.description = u.description AND t.category_id IS NOT NULL AND t.id != u.id \
           WHERE bc.transaction_id IS NULL \
           GROUP BY u.id, t.category_id \
         ), \
         best_desc AS ( \
           SELECT transaction_id, suggested_category_id \
           FROM desc_suggestions ds \
           WHERE freq = (SELECT MAX(freq) FROM desc_suggestions WHERE transaction_id = ds.transaction_id) \
           GROUP BY transaction_id \
         ) \
         SELECT transaction_id, suggested_category_id FROM best_cp \
         UNION ALL \
         SELECT transaction_id, suggested_category_id FROM best_desc",
    )
    .bind::<Text, _>(&start)
    .bind::<Text, _>(&end)
    .load::<CategorySuggestion>(conn)?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    use crate::db::schema::{accounts, categories, transaction_types};
    use crate::models::account::NewAccount;
    use crate::models::category::Category;
    use crate::models::transaction::Transaction;
    use crate::services::counterparty_service;

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
    fn test_list_by_month_includes_counterparty_name() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        let cp = counterparty_service::find_or_create(
            conn,
            Some("BE00 1234 5678 9012"),
            "Colruyt",
            None,
            None,
            None,
            None,
        )
        .unwrap();

        // Transaction WITH counterparty
        let with_cp = NewTransaction {
            account_id,
            counterparty_id: Some(cp.id),
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-15",
            value_date: "2024-12-15",
            statement_number: None,
            transaction_number: None,
            amount_cents: -3000,
            currency: "EUR",
            description: "COLRUYT",
            communication: None,
            import_hash: "cp_name_with",
        };
        insert(conn, &with_cp).unwrap();

        // Transaction WITHOUT counterparty
        let without_cp = NewTransaction {
            account_id,
            counterparty_id: None,
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-16",
            value_date: "2024-12-16",
            statement_number: None,
            transaction_number: None,
            amount_cents: -1000,
            currency: "EUR",
            description: "ATM",
            communication: None,
            import_hash: "cp_name_without",
        };
        insert(conn, &without_cp).unwrap();

        let results = list_by_month(conn, 2024, 12).unwrap();
        assert_eq!(results.len(), 2);

        // Results are ordered by date DESC, so "2024-12-16" comes first
        let atm = &results[0];
        assert_eq!(atm.description, "ATM");
        assert_eq!(atm.counterparty_name, None);

        let colruyt = &results[1];
        assert_eq!(colruyt.description, "COLRUYT");
        assert_eq!(colruyt.counterparty_name, Some("Colruyt".to_string()));
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

    #[test]
    fn test_suggest_by_counterparty() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        let cp = counterparty_service::find_or_create(
            conn, Some("BE00 1234 5678 9012"), "Colruyt", None, None, None, None,
        ).unwrap();

        let food: Category = categories::table
            .filter(categories::name.eq("Food & Groceries"))
            .first(conn)
            .unwrap();

        // Categorized tx with same counterparty (different month)
        let old = NewTransaction {
            account_id,
            counterparty_id: Some(cp.id),
            category_id: Some(food.id),
            transaction_type_id: type_id,
            accounting_date: "2024-11-15",
            value_date: "2024-11-15",
            statement_number: None,
            transaction_number: None,
            amount_cents: -3000,
            currency: "EUR",
            description: "COLRUYT",
            communication: None,
            import_hash: "sug_cp_old",
        };
        insert(conn, &old).unwrap();

        // Uncategorized tx with same counterparty in target month
        let new = NewTransaction {
            account_id,
            counterparty_id: Some(cp.id),
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-10",
            value_date: "2024-12-10",
            statement_number: None,
            transaction_number: None,
            amount_cents: -4500,
            currency: "EUR",
            description: "COLRUYT",
            communication: None,
            import_hash: "sug_cp_new",
        };
        insert(conn, &new).unwrap();

        let suggestions = suggest_categories(conn, 2024, 12).unwrap();
        assert_eq!(suggestions.len(), 1);

        let uncat_tx: Transaction = transactions::table
            .filter(transactions::import_hash.eq("sug_cp_new"))
            .first(conn)
            .unwrap();
        assert_eq!(suggestions[0].transaction_id, uncat_tx.id);
        assert_eq!(suggestions[0].suggested_category_id, food.id);
    }

    #[test]
    fn test_suggest_by_description_fallback() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        let transport: Category = categories::table
            .filter(categories::name.eq("Transport"))
            .first(conn)
            .unwrap();

        // Categorized tx with same description, no counterparty
        let old = NewTransaction {
            account_id,
            counterparty_id: None,
            category_id: Some(transport.id),
            transaction_type_id: type_id,
            accounting_date: "2024-11-05",
            value_date: "2024-11-05",
            statement_number: None,
            transaction_number: None,
            amount_cents: -2500,
            currency: "EUR",
            description: "NMBS TICKET",
            communication: None,
            import_hash: "sug_desc_old",
        };
        insert(conn, &old).unwrap();

        // Uncategorized tx with same description
        let new = NewTransaction {
            account_id,
            counterparty_id: None,
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-20",
            value_date: "2024-12-20",
            statement_number: None,
            transaction_number: None,
            amount_cents: -2500,
            currency: "EUR",
            description: "NMBS TICKET",
            communication: None,
            import_hash: "sug_desc_new",
        };
        insert(conn, &new).unwrap();

        let suggestions = suggest_categories(conn, 2024, 12).unwrap();
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].suggested_category_id, transport.id);
    }

    #[test]
    fn test_counterparty_takes_priority() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        let cp = counterparty_service::find_or_create(
            conn, Some("BE99 0000 1111 2222"), "Merchant", None, None, None, None,
        ).unwrap();

        let food: Category = categories::table
            .filter(categories::name.eq("Food & Groceries"))
            .first(conn)
            .unwrap();
        let transport: Category = categories::table
            .filter(categories::name.eq("Transport"))
            .first(conn)
            .unwrap();

        // Counterparty-based: categorized as Food
        let old_cp = NewTransaction {
            account_id,
            counterparty_id: Some(cp.id),
            category_id: Some(food.id),
            transaction_type_id: type_id,
            accounting_date: "2024-11-01",
            value_date: "2024-11-01",
            statement_number: None,
            transaction_number: None,
            amount_cents: -1000,
            currency: "EUR",
            description: "SHARED DESC",
            communication: None,
            import_hash: "prio_cp",
        };
        insert(conn, &old_cp).unwrap();

        // Description-based: categorized as Transport (different counterparty)
        let old_desc = NewTransaction {
            account_id,
            counterparty_id: None,
            category_id: Some(transport.id),
            transaction_type_id: type_id,
            accounting_date: "2024-11-02",
            value_date: "2024-11-02",
            statement_number: None,
            transaction_number: None,
            amount_cents: -1000,
            currency: "EUR",
            description: "SHARED DESC",
            communication: None,
            import_hash: "prio_desc",
        };
        insert(conn, &old_desc).unwrap();

        // Uncategorized tx with both counterparty and matching description
        let new = NewTransaction {
            account_id,
            counterparty_id: Some(cp.id),
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-15",
            value_date: "2024-12-15",
            statement_number: None,
            transaction_number: None,
            amount_cents: -1000,
            currency: "EUR",
            description: "SHARED DESC",
            communication: None,
            import_hash: "prio_new",
        };
        insert(conn, &new).unwrap();

        let suggestions = suggest_categories(conn, 2024, 12).unwrap();
        assert_eq!(suggestions.len(), 1);
        // Counterparty match (Food) takes priority over description match (Transport)
        assert_eq!(suggestions[0].suggested_category_id, food.id);
    }

    #[test]
    fn test_most_frequent_wins() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        let cp = counterparty_service::find_or_create(
            conn, Some("BE77 5555 6666 7777"), "FreqMerchant", None, None, None, None,
        ).unwrap();

        let food: Category = categories::table
            .filter(categories::name.eq("Food & Groceries"))
            .first(conn)
            .unwrap();
        let transport: Category = categories::table
            .filter(categories::name.eq("Transport"))
            .first(conn)
            .unwrap();

        // 3 txs categorized as Food for this counterparty
        for i in 0..3 {
            let tx = NewTransaction {
                account_id,
                counterparty_id: Some(cp.id),
                category_id: Some(food.id),
                transaction_type_id: type_id,
                accounting_date: "2024-10-01",
                value_date: "2024-10-01",
                statement_number: None,
                transaction_number: None,
                amount_cents: -1000,
                currency: "EUR",
                description: "FREQ",
                communication: None,
                import_hash: &format!("freq_food_{}", i),
            };
            insert(conn, &tx).unwrap();
        }

        // 1 tx categorized as Transport for same counterparty
        let tx = NewTransaction {
            account_id,
            counterparty_id: Some(cp.id),
            category_id: Some(transport.id),
            transaction_type_id: type_id,
            accounting_date: "2024-10-05",
            value_date: "2024-10-05",
            statement_number: None,
            transaction_number: None,
            amount_cents: -1000,
            currency: "EUR",
            description: "FREQ",
            communication: None,
            import_hash: "freq_transport_0",
        };
        insert(conn, &tx).unwrap();

        // Uncategorized tx with same counterparty
        let new = NewTransaction {
            account_id,
            counterparty_id: Some(cp.id),
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-01",
            value_date: "2024-12-01",
            statement_number: None,
            transaction_number: None,
            amount_cents: -1000,
            currency: "EUR",
            description: "FREQ",
            communication: None,
            import_hash: "freq_new",
        };
        insert(conn, &new).unwrap();

        let suggestions = suggest_categories(conn, 2024, 12).unwrap();
        assert_eq!(suggestions.len(), 1);
        // Food (3 occurrences) wins over Transport (1 occurrence)
        assert_eq!(suggestions[0].suggested_category_id, food.id);
    }

    #[test]
    fn test_no_match_returns_empty() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        // Uncategorized tx with unique description and no counterparty
        let new = NewTransaction {
            account_id,
            counterparty_id: None,
            category_id: None,
            transaction_type_id: type_id,
            accounting_date: "2024-12-25",
            value_date: "2024-12-25",
            statement_number: None,
            transaction_number: None,
            amount_cents: -9999,
            currency: "EUR",
            description: "TOTALLY UNIQUE DESCRIPTION",
            communication: None,
            import_hash: "no_match",
        };
        insert(conn, &new).unwrap();

        let suggestions = suggest_categories(conn, 2024, 12).unwrap();
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_ignores_already_categorized() {
        let conn = &mut establish_test_connection();
        let (account_id, type_id) = setup(conn);

        let cp = counterparty_service::find_or_create(
            conn, Some("BE11 2222 3333 4444"), "CatMerchant", None, None, None, None,
        ).unwrap();

        let food: Category = categories::table
            .filter(categories::name.eq("Food & Groceries"))
            .first(conn)
            .unwrap();

        // Previously categorized tx
        let old = NewTransaction {
            account_id,
            counterparty_id: Some(cp.id),
            category_id: Some(food.id),
            transaction_type_id: type_id,
            accounting_date: "2024-11-10",
            value_date: "2024-11-10",
            statement_number: None,
            transaction_number: None,
            amount_cents: -2000,
            currency: "EUR",
            description: "CAT MERCHANT",
            communication: None,
            import_hash: "ignore_old",
        };
        insert(conn, &old).unwrap();

        // Already categorized tx in target month — should NOT appear in suggestions
        let already_cat = NewTransaction {
            account_id,
            counterparty_id: Some(cp.id),
            category_id: Some(food.id),
            transaction_type_id: type_id,
            accounting_date: "2024-12-10",
            value_date: "2024-12-10",
            statement_number: None,
            transaction_number: None,
            amount_cents: -2000,
            currency: "EUR",
            description: "CAT MERCHANT",
            communication: None,
            import_hash: "ignore_already",
        };
        insert(conn, &already_cat).unwrap();

        let suggestions = suggest_categories(conn, 2024, 12).unwrap();
        assert!(suggestions.is_empty());
    }
}
