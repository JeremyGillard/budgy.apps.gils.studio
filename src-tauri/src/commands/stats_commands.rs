use tauri::State;

use crate::commands::import_commands::DbConn;
use crate::error::BudgyError;
use crate::services::stats_service::{
    self, AvgMonthlyCategorySpend, CategoryBreakdown, DailySummary, ImportedMonth, MonthlySummary,
    YearlyTopCategories,
};

#[tauri::command]
pub fn monthly_summary(
    db: State<DbConn>,
    year: i32,
    month: u32,
) -> Result<MonthlySummary, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    stats_service::monthly_summary(conn, year, month)
}

#[tauri::command]
pub fn category_breakdown(
    db: State<DbConn>,
    year: i32,
    month: u32,
) -> Result<Vec<CategoryBreakdown>, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    stats_service::category_breakdown(conn, year, month)
}

#[tauri::command]
pub fn daily_summary(
    db: State<DbConn>,
    year: i32,
    month: u32,
) -> Result<Vec<DailySummary>, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    stats_service::daily_summary(conn, year, month)
}

#[tauri::command]
pub fn get_imported_months(
    db: State<DbConn>,
) -> Result<Vec<ImportedMonth>, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    stats_service::imported_months(conn)
}

#[tauri::command]
pub fn yearly_earnings(
    db: State<DbConn>,
    year: i32,
) -> Result<YearlyTopCategories, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    stats_service::yearly_earnings_by_category(conn, year)
}

#[tauri::command]
pub fn yearly_expenses(
    db: State<DbConn>,
    year: i32,
) -> Result<YearlyTopCategories, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    stats_service::yearly_expenses_by_category(conn, year)
}

#[tauri::command]
pub fn avg_monthly_spend(
    db: State<DbConn>,
) -> Result<Vec<AvgMonthlyCategorySpend>, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    stats_service::avg_monthly_category_spend(conn)
}
