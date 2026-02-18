use std::sync::Mutex;
use tauri::State;

use crate::error::BudgyError;
use crate::services::import_service::{self, ImportResult};

pub struct DbConn(pub Mutex<diesel::SqliteConnection>);

#[tauri::command]
pub fn import_csv(
    db: State<DbConn>,
    file_paths: Vec<String>,
) -> Result<Vec<ImportResult>, BudgyError> {
    let mut conn = db.0.lock().map_err(|e| BudgyError::General(e.to_string()))?;
    let mut results = Vec::new();

    for file_path in &file_paths {
        let content = std::fs::read(file_path)?;
        let filename = std::path::Path::new(file_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| file_path.clone());
        results.push(import_service::import_csv(&mut conn, &filename, &content)?);
    }

    Ok(results)
}
