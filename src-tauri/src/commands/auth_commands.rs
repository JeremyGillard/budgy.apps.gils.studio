use serde::Serialize;
use tauri::State;

use crate::commands::import_commands::DbConn;
use crate::db::connection::{
    database_exists, establish_encrypted_connection, is_database_unencrypted, migrate_to_encrypted,
    run_migrations,
};
use crate::error::BudgyError;

pub struct DbPath(pub String);

#[derive(Serialize)]
pub enum DbStatus {
    FirstTime,
    Unencrypted,
    Encrypted,
}

#[tauri::command]
pub fn get_db_status(db_path: State<DbPath>) -> Result<DbStatus, BudgyError> {
    if !database_exists(&db_path.0) {
        return Ok(DbStatus::FirstTime);
    }
    if is_database_unencrypted(&db_path.0)? {
        return Ok(DbStatus::Unencrypted);
    }
    Ok(DbStatus::Encrypted)
}

#[tauri::command]
pub fn unlock_db(
    db: State<DbConn>,
    db_path: State<DbPath>,
    password: String,
) -> Result<(), BudgyError> {
    let mut conn = establish_encrypted_connection(&db_path.0, &password)?;
    run_migrations(&mut conn)?;

    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    *guard = Some(conn);
    Ok(())
}

#[tauri::command]
pub fn setup_encryption(
    db: State<DbConn>,
    db_path: State<DbPath>,
    password: String,
) -> Result<(), BudgyError> {
    // If there's an existing unencrypted DB, migrate it first
    if database_exists(&db_path.0) && is_database_unencrypted(&db_path.0)? {
        migrate_to_encrypted(&db_path.0, &password)?;
    }

    // Now open (or create) the encrypted database
    let mut conn = establish_encrypted_connection(&db_path.0, &password)?;
    run_migrations(&mut conn)?;

    let mut guard = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    *guard = Some(conn);
    Ok(())
}
