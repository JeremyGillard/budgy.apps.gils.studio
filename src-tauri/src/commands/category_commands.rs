use tauri::State;

use crate::commands::import_commands::DbConn;
use crate::error::BudgyError;
use crate::models::category::{Category, CreateCategoryInput, UpdateCategory};
use crate::services::category_service;

#[tauri::command]
pub fn list_categories(db: State<DbConn>) -> Result<Vec<Category>, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    category_service::list_all(conn)
}

#[tauri::command]
pub fn get_category(db: State<DbConn>, id: i32) -> Result<Option<Category>, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    category_service::get_by_id(conn, id)
}

#[tauri::command]
pub fn create_category(
    db: State<DbConn>,
    input: CreateCategoryInput,
) -> Result<Category, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    category_service::create(conn, &input)
}

#[tauri::command]
pub fn update_category(
    db: State<DbConn>,
    id: i32,
    changes: UpdateCategory,
) -> Result<Category, BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    category_service::update(conn, id, &changes)
}

#[tauri::command]
pub fn delete_category(db: State<DbConn>, id: i32) -> Result<(), BudgyError> {
    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let conn = guard.as_mut().ok_or(BudgyError::DatabaseLocked)?;
    category_service::delete(conn, id)
}
