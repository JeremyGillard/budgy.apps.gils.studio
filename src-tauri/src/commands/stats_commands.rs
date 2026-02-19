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
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    stats_service::monthly_summary(&mut conn, year, month)
}

#[tauri::command]
pub fn category_breakdown(
    db: State<DbConn>,
    year: i32,
    month: u32,
) -> Result<Vec<CategoryBreakdown>, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    stats_service::category_breakdown(&mut conn, year, month)
}

#[tauri::command]
pub fn daily_summary(
    db: State<DbConn>,
    year: i32,
    month: u32,
) -> Result<Vec<DailySummary>, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    stats_service::daily_summary(&mut conn, year, month)
}

#[tauri::command]
pub fn get_imported_months(
    db: State<DbConn>,
) -> Result<Vec<ImportedMonth>, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    stats_service::imported_months(&mut conn)
}

#[tauri::command]
pub fn yearly_earnings(
    db: State<DbConn>,
    year: i32,
) -> Result<YearlyTopCategories, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    stats_service::yearly_earnings_by_category(&mut conn, year)
}

#[tauri::command]
pub fn yearly_expenses(
    db: State<DbConn>,
    year: i32,
) -> Result<YearlyTopCategories, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    stats_service::yearly_expenses_by_category(&mut conn, year)
}

#[tauri::command]
pub fn avg_monthly_spend(
    db: State<DbConn>,
) -> Result<Vec<AvgMonthlyCategorySpend>, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    stats_service::avg_monthly_category_spend(&mut conn)
}
