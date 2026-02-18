use tauri::State;

use crate::commands::import_commands::DbConn;
use crate::error::BudgyError;
use crate::models::account::Account;
use crate::services::account_service;

#[tauri::command]
pub fn list_accounts(db: State<DbConn>) -> Result<Vec<Account>, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    account_service::list_all(&mut conn)
}
