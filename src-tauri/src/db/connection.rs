use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::error::BudgyError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn establish_connection(database_url: &str) -> Result<SqliteConnection, BudgyError> {
    let mut conn = SqliteConnection::establish(database_url)
        .map_err(|e| BudgyError::General(format!("Cannot connect to database: {}", e)))?;
    // Enable WAL mode and foreign keys
    diesel::sql_query("PRAGMA journal_mode=WAL;")
        .execute(&mut conn)
        .ok();
    diesel::sql_query("PRAGMA foreign_keys=ON;")
        .execute(&mut conn)
        .ok();
    Ok(conn)
}

pub fn run_migrations(conn: &mut SqliteConnection) -> Result<(), BudgyError> {
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| BudgyError::Migration(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
pub fn establish_test_connection() -> SqliteConnection {
    let mut conn = SqliteConnection::establish(":memory:")
        .expect("Failed to create in-memory database");
    diesel::sql_query("PRAGMA foreign_keys=ON;")
        .execute(&mut conn)
        .ok();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    conn
}
