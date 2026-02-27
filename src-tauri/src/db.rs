use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Mutex;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct DbState(pub Mutex<SqliteConnection>);

pub fn establish_connection() -> SqliteConnection {
    let db_path = get_db_path();
    let mut conn = SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path));

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");

    conn
}

fn get_db_path() -> String {
    #[cfg(test)]
    {
        ":memory:".to_string()
    }
    #[cfg(not(test))]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let dir = format!("{}/.local/share/studio.gils.apps.budgy", home);
        std::fs::create_dir_all(&dir).ok();
        format!("{}/budgy.db", dir)
    }
}
