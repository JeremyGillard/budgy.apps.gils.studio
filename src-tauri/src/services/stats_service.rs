use diesel::prelude::*;
use diesel::sql_types::{Integer, Nullable, Text};
use serde::Serialize;

use crate::db::schema::transactions;
use crate::error::BudgyError;

#[derive(Debug, Serialize)]
pub struct MonthlySummary {
    pub year: i32,
    pub month: u32,
    pub total_income_cents: i64,
    pub total_expenses_cents: i64,
    pub net_cents: i64,
    pub transaction_count: usize,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct CategoryBreakdown {
    #[diesel(sql_type = Nullable<Text>)]
    pub category_name: Option<String>,
    #[diesel(sql_type = Integer)]
    pub total_cents: i32,
    #[diesel(sql_type = Integer)]
    pub count: i32,
}

pub fn monthly_summary(
    conn: &mut SqliteConnection,
    year: i32,
    month: u32,
) -> Result<MonthlySummary, BudgyError> {
    let start = format!("{:04}-{:02}-01", year, month);
    let end = format!("{:04}-{:02}-31", year, month);

    let amounts: Vec<i32> = transactions::table
        .filter(transactions::accounting_date.ge(&start))
        .filter(transactions::accounting_date.le(&end))
        .select(transactions::amount_cents)
        .load(conn)?;

    let total_income_cents: i64 = amounts.iter().filter(|a| **a > 0).map(|a| *a as i64).sum();
    let total_expenses_cents: i64 = amounts.iter().filter(|a| **a < 0).map(|a| *a as i64).sum();

    Ok(MonthlySummary {
        year,
        month,
        total_income_cents,
        total_expenses_cents,
        net_cents: total_income_cents + total_expenses_cents,
        transaction_count: amounts.len(),
    })
}

pub fn category_breakdown(
    conn: &mut SqliteConnection,
    year: i32,
    month: u32,
) -> Result<Vec<CategoryBreakdown>, BudgyError> {
    let start = format!("{:04}-{:02}-01", year, month);
    let end = format!("{:04}-{:02}-31", year, month);

    let results = diesel::sql_query(
        "SELECT c.name as category_name, \
         CAST(SUM(t.amount_cents) AS INTEGER) as total_cents, \
         CAST(COUNT(*) AS INTEGER) as count \
         FROM transactions t \
         LEFT JOIN categories c ON t.category_id = c.id \
         WHERE t.accounting_date >= ? AND t.accounting_date <= ? \
         GROUP BY t.category_id \
         ORDER BY total_cents ASC",
    )
    .bind::<Text, _>(&start)
    .bind::<Text, _>(&end)
    .load::<CategoryBreakdown>(conn)?;

    Ok(results)
}

#[derive(Debug, Serialize)]
pub struct DailySummary {
    pub date: String,
    pub total_income_cents: i64,
    pub total_expenses_cents: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct ImportedMonth {
    #[diesel(sql_type = Integer)]
    pub year: i32,
    #[diesel(sql_type = Integer)]
    pub month: i32,
}

pub fn imported_months(conn: &mut SqliteConnection) -> Result<Vec<ImportedMonth>, BudgyError> {
    let results = diesel::sql_query(
        "SELECT CAST(strftime('%Y', accounting_date) AS INTEGER) as year, \
         CAST(strftime('%m', accounting_date) AS INTEGER) as month \
         FROM transactions \
         GROUP BY year, month \
         ORDER BY year, month",
    )
    .load::<ImportedMonth>(conn)?;

    Ok(results)
}

pub fn daily_summary(
    conn: &mut SqliteConnection,
    year: i32,
    month: u32,
) -> Result<Vec<DailySummary>, BudgyError> {
    let start = format!("{:04}-{:02}-01", year, month);
    let end = format!("{:04}-{:02}-31", year, month);

    let rows: Vec<(String, i32)> = transactions::table
        .filter(transactions::accounting_date.ge(&start))
        .filter(transactions::accounting_date.le(&end))
        .select((transactions::accounting_date, transactions::amount_cents))
        .load(conn)?;

    let mut by_date: std::collections::BTreeMap<String, (i64, i64)> =
        std::collections::BTreeMap::new();
    for (date, amount) in rows {
        let entry = by_date.entry(date).or_insert((0, 0));
        if amount > 0 {
            entry.0 += amount as i64;
        } else {
            entry.1 += amount as i64;
        }
    }

    Ok(by_date
        .into_iter()
        .map(|(date, (income, expenses))| DailySummary {
            date,
            total_income_cents: income,
            total_expenses_cents: expenses,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    use crate::services::import_service;

    fn sample_csv() -> Vec<u8> {
        std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../.samples/BE34 0634 5590 5590 2025-08-08 9-59-57 1.csv"
        ))
        .unwrap()
    }

    #[test]
    fn test_monthly_summary_with_data() {
        let conn = &mut establish_test_connection();
        import_service::import_csv(conn, "test.csv", &sample_csv()).unwrap();

        let summary = monthly_summary(conn, 2024, 12).unwrap();
        assert!(summary.transaction_count > 0, "December should have transactions");
        assert!(summary.total_expenses_cents < 0, "Should have expenses");
    }

    #[test]
    fn test_monthly_summary_empty_month() {
        let conn = &mut establish_test_connection();
        let summary = monthly_summary(conn, 2020, 1).unwrap();
        assert_eq!(summary.transaction_count, 0);
        assert_eq!(summary.total_income_cents, 0);
        assert_eq!(summary.total_expenses_cents, 0);
        assert_eq!(summary.net_cents, 0);
    }

    #[test]
    fn test_daily_summary_with_data() {
        let conn = &mut establish_test_connection();
        import_service::import_csv(conn, "test.csv", &sample_csv()).unwrap();

        let result = daily_summary(conn, 2024, 12).unwrap();
        assert!(!result.is_empty(), "December should have daily entries");
        for day in &result {
            assert!(day.date.starts_with("2024-12-"));
        }
    }

    #[test]
    fn test_daily_summary_empty_month() {
        let conn = &mut establish_test_connection();
        let result = daily_summary(conn, 2020, 1).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_category_breakdown() {
        let conn = &mut establish_test_connection();
        import_service::import_csv(conn, "test.csv", &sample_csv()).unwrap();

        let breakdown = category_breakdown(conn, 2024, 12).unwrap();
        // All uncategorized at this point, so should have 1 group with null category
        assert!(!breakdown.is_empty());
    }

    #[test]
    fn test_imported_months() {
        let conn = &mut establish_test_connection();
        import_service::import_csv(conn, "test.csv", &sample_csv()).unwrap();

        let months = imported_months(conn).unwrap();
        assert!(!months.is_empty(), "Should have imported months");
        // Verify all entries have valid year/month
        for m in &months {
            assert!(m.year >= 2000 && m.year <= 2100);
            assert!(m.month >= 1 && m.month <= 12);
        }
        // Sample data includes December 2024
        assert!(
            months.iter().any(|m| m.year == 2024 && m.month == 12),
            "Should contain December 2024"
        );
    }
}
