use serde::Serialize;
use tauri::State;

use crate::db::DbState;
use crate::models::Transaction;
use crate::services::transaction_service;

#[derive(Serialize)]
pub struct PaginatedResponse {
    pub data: Vec<Transaction>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[tauri::command]
pub fn list_transactions(
    state: State<DbState>,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Result<PaginatedResponse, String> {
    let page = page.unwrap_or(1).max(1);
    let per_page = per_page.unwrap_or(25).clamp(1, 100);

    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let result = transaction_service::list_paginated(&mut conn, page, per_page)
        .map_err(|e| e.to_string())?;

    Ok(result)
}
