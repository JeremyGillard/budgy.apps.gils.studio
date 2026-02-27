use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::commands::transactions::PaginatedResponse;
use crate::models::Transaction;
use crate::schema::transactions;

pub fn list_paginated(
    conn: &mut SqliteConnection,
    page: i64,
    per_page: i64,
) -> Result<PaginatedResponse, diesel::result::Error> {
    let offset = (page - 1) * per_page;

    let total: i64 = transactions::table
        .count()
        .get_result(conn)?;

    let data: Vec<Transaction> = transactions::table
        .order((transactions::accounting_date.desc(), transactions::sequence_number.desc()))
        .limit(per_page)
        .offset(offset)
        .load(conn)?;

    let total_pages = (total + per_page - 1) / per_page;

    Ok(PaginatedResponse {
        data,
        total,
        page,
        per_page,
        total_pages,
    })
}
