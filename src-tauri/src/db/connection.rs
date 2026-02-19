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

pub fn establish_encrypted_connection(
    database_url: &str,
    password: &str,
) -> Result<SqliteConnection, BudgyError> {
    let mut conn = SqliteConnection::establish(database_url)
        .map_err(|e| BudgyError::General(format!("Cannot connect to database: {}", e)))?;

    // Set the encryption key — must be the very first statement
    let escaped = password.replace('\'', "''");
    diesel::sql_query(format!("PRAGMA key = '{}';", escaped))
        .execute(&mut conn)
        .map_err(|e| BudgyError::General(format!("Failed to set encryption key: {}", e)))?;

    // Verify the key is correct by reading the schema
    diesel::sql_query("SELECT count(*) FROM sqlite_master;")
        .execute(&mut conn)
        .map_err(|_| BudgyError::WrongPassword)?;

    // Enable WAL mode and foreign keys
    diesel::sql_query("PRAGMA journal_mode=WAL;")
        .execute(&mut conn)
        .ok();
    diesel::sql_query("PRAGMA foreign_keys=ON;")
        .execute(&mut conn)
        .ok();

    Ok(conn)
}

pub fn database_exists(db_path: &str) -> bool {
    let path = std::path::Path::new(db_path);
    path.exists() && path.metadata().map(|m| m.len() > 0).unwrap_or(false)
}

pub fn is_database_unencrypted(db_path: &str) -> Result<bool, BudgyError> {
    let mut file = std::fs::File::open(db_path)?;
    let mut header = [0u8; 16];
    use std::io::Read;
    let bytes_read = file.read(&mut header)?;
    if bytes_read < 16 {
        return Ok(false);
    }
    // Unencrypted SQLite files start with "SQLite format 3\0"
    Ok(&header[..16] == b"SQLite format 3\0")
}

pub fn migrate_to_encrypted(db_path: &str, password: &str) -> Result<(), BudgyError> {
    // Open the existing unencrypted database
    let mut conn = establish_connection(db_path)?;

    let encrypted_path = format!("{}.encrypted", db_path);
    let escaped = password.replace('\'', "''");

    // Attach an encrypted database and export data into it
    diesel::sql_query(format!(
        "ATTACH DATABASE '{}' AS encrypted KEY '{}';",
        encrypted_path, escaped
    ))
    .execute(&mut conn)
    .map_err(|e| BudgyError::General(format!("Failed to attach encrypted db: {}", e)))?;

    diesel::sql_query("SELECT sqlcipher_export('encrypted');")
        .execute(&mut conn)
        .map_err(|e| BudgyError::General(format!("Failed to export to encrypted db: {}", e)))?;

    diesel::sql_query("DETACH DATABASE encrypted;")
        .execute(&mut conn)
        .map_err(|e| BudgyError::General(format!("Failed to detach encrypted db: {}", e)))?;

    drop(conn);

    // Swap files: backup original, replace with encrypted
    let backup_path = format!("{}.bak", db_path);
    std::fs::rename(db_path, &backup_path)?;
    std::fs::rename(&encrypted_path, db_path)?;

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypted_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db").to_string_lossy().to_string();
        let password = "test-password-123";

        // Create an encrypted database, insert data, close it
        {
            let mut conn = establish_encrypted_connection(&db_path, password).unwrap();
            run_migrations(&mut conn).unwrap();
            diesel::sql_query(
                "INSERT INTO accounts (iban, label, created_at, updated_at) \
                 VALUES ('BE00', 'Test', datetime('now'), datetime('now'))"
            )
            .execute(&mut conn)
            .unwrap();
        }

        // Re-open with same password — data should be there
        {
            let mut conn = establish_encrypted_connection(&db_path, password).unwrap();
            let count: i32 = diesel::sql_query("SELECT count(*) as cnt FROM accounts")
                .get_result::<CountResult>(&mut conn)
                .unwrap()
                .cnt;
            assert!(count >= 1);
        }
    }

    #[test]
    fn test_wrong_password_fails() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db").to_string_lossy().to_string();

        // Create encrypted DB with one password
        {
            let mut conn = establish_encrypted_connection(&db_path, "correct").unwrap();
            run_migrations(&mut conn).unwrap();
        }

        // Try to open with wrong password
        let result = establish_encrypted_connection(&db_path, "wrong");
        assert!(
            matches!(&result, Err(BudgyError::WrongPassword)),
            "Expected WrongPassword error"
        );
        drop(result);
    }

    #[test]
    fn test_database_exists() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db").to_string_lossy().to_string();

        assert!(!database_exists(&db_path));

        // Create a file with content
        std::fs::write(&db_path, b"some data").unwrap();
        assert!(database_exists(&db_path));

        // Empty file should return false
        let empty_path = dir.path().join("empty.db").to_string_lossy().to_string();
        std::fs::write(&empty_path, b"").unwrap();
        assert!(!database_exists(&empty_path));
    }

    #[test]
    fn test_is_database_unencrypted() {
        let dir = tempfile::tempdir().unwrap();

        // Create an unencrypted SQLite DB
        let plain_path = dir.path().join("plain.db").to_string_lossy().to_string();
        {
            let mut conn = establish_connection(&plain_path).unwrap();
            run_migrations(&mut conn).unwrap();
        }
        assert!(is_database_unencrypted(&plain_path).unwrap());

        // Create an encrypted DB
        let enc_path = dir.path().join("enc.db").to_string_lossy().to_string();
        {
            let mut conn = establish_encrypted_connection(&enc_path, "pass").unwrap();
            run_migrations(&mut conn).unwrap();
        }
        assert!(!is_database_unencrypted(&enc_path).unwrap());
    }

    #[test]
    fn test_migrate_to_encrypted() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db").to_string_lossy().to_string();
        let password = "migrate-pass";

        // Create unencrypted DB with data
        {
            let mut conn = establish_connection(&db_path).unwrap();
            run_migrations(&mut conn).unwrap();
            diesel::sql_query(
                "INSERT INTO accounts (iban, label, created_at, updated_at) \
                 VALUES ('BE00', 'Test', datetime('now'), datetime('now'))"
            )
            .execute(&mut conn)
            .unwrap();
        }
        assert!(is_database_unencrypted(&db_path).unwrap());

        // Migrate to encrypted
        migrate_to_encrypted(&db_path, password).unwrap();

        // The file should now be encrypted (not plain SQLite header)
        assert!(!is_database_unencrypted(&db_path).unwrap());

        // Backup should exist
        assert!(std::path::Path::new(&format!("{}.bak", db_path)).exists());

        // Should be openable with correct password and data preserved
        {
            let mut conn = establish_encrypted_connection(&db_path, password).unwrap();
            let count: i32 = diesel::sql_query("SELECT count(*) as cnt FROM accounts")
                .get_result::<CountResult>(&mut conn)
                .unwrap()
                .cnt;
            assert!(count >= 1);
        }
    }

    #[test]
    fn test_encrypted_db_unreadable_without_password() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db").to_string_lossy().to_string();

        // Create an encrypted database with data
        {
            let mut conn = establish_encrypted_connection(&db_path, "secret").unwrap();
            run_migrations(&mut conn).unwrap();
            diesel::sql_query(
                "INSERT INTO accounts (iban, label, created_at, updated_at) \
                 VALUES ('BE00', 'Test', datetime('now'), datetime('now'))",
            )
            .execute(&mut conn)
            .unwrap();
        }

        // Open with no password — should connect but fail to read
        let mut conn = establish_connection(&db_path).unwrap();
        let result = diesel::sql_query("SELECT count(*) FROM sqlite_master;").execute(&mut conn);
        assert!(result.is_err(), "Expected error reading encrypted DB without password");
    }

    // Helper for raw SQL count queries in tests
    use diesel::sql_types::Integer;
    #[derive(QueryableByName)]
    struct CountResult {
        #[diesel(sql_type = Integer)]
        cnt: i32,
    }
}
