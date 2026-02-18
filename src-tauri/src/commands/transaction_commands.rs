use tauri::State;

use crate::commands::import_commands::DbConn;
use crate::error::BudgyError;
use crate::models::transaction::Transaction;
use crate::services::transaction_service;

#[tauri::command]
pub fn list_transactions_by_month(
    db: State<DbConn>,
    year: i32,
    month: u32,
) -> Result<Vec<Transaction>, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    transaction_service::list_by_month(&mut conn, year, month)
}

#[tauri::command]
pub fn list_transactions_by_account(
    db: State<DbConn>,
    account_id: i32,
) -> Result<Vec<Transaction>, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    transaction_service::list_by_account(&mut conn, account_id)
}

#[tauri::command]
pub fn categorize_transaction(
    db: State<DbConn>,
    transaction_id: i32,
    category_id: i32,
) -> Result<(), BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    transaction_service::categorize(&mut conn, transaction_id, category_id)
}
