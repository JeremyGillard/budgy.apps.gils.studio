use tauri::State;

use crate::commands::import_commands::DbConn;
use crate::error::BudgyError;
use crate::services::stats_service::{self, CategoryBreakdown, MonthlySummary};

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
