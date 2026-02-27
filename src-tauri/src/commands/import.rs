use serde::Serialize;
use tauri::State;

use crate::db::DbState;
use crate::services::import_service;

#[derive(Serialize)]
pub struct ImportResult {
    pub file_name: String,
    pub imported_count: usize,
    pub skipped_count: usize,
    pub error: Option<String>,
}

#[tauri::command]
pub fn import_csv(
    state: State<DbState>,
    file_paths: Vec<String>,
) -> Result<Vec<ImportResult>, String> {
    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut results = Vec::new();

    for path in &file_paths {
        match import_service::import_file(&mut conn, path) {
            Ok(result) => results.push(ImportResult {
                file_name: result.file_name,
                imported_count: result.imported_count,
                skipped_count: result.skipped_count,
                error: None,
            }),
            Err(e) => results.push(ImportResult {
                file_name: path.clone(),
                imported_count: 0,
                skipped_count: 0,
                error: Some(e.to_string()),
            }),
        }
    }

    Ok(results)
}
