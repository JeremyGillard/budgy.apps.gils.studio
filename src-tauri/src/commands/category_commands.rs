use tauri::State;

use crate::commands::import_commands::DbConn;
use crate::error::BudgyError;
use crate::models::category::Category;
use crate::services::category_service;

#[tauri::command]
pub fn list_categories(db: State<DbConn>) -> Result<Vec<Category>, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    category_service::list_all(&mut conn)
}
