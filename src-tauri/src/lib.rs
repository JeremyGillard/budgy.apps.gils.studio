pub mod commands;
pub mod csv;
pub mod db;
pub mod error;
pub mod models;
pub mod services;

use std::sync::Mutex;

use commands::import_commands::DbConn;
use db::connection::{establish_connection, run_migrations};
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");

            let db_path = app_dir.join("budgy.db");
            let db_url = db_path.to_string_lossy().to_string();

            let mut conn = establish_connection(&db_url)
                .expect("Failed to establish database connection");
            run_migrations(&mut conn).expect("Failed to run migrations");

            app.manage(DbConn(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::import_commands::import_csv,
            commands::transaction_commands::list_transactions_by_month,
            commands::transaction_commands::list_transactions_by_account,
            commands::transaction_commands::categorize_transaction,
            commands::transaction_commands::bulk_categorize_transactions,
            commands::transaction_commands::get_categorization_stats,
            commands::transaction_commands::get_category_suggestions,
            commands::account_commands::list_accounts,
            commands::category_commands::list_categories,
            commands::stats_commands::monthly_summary,
            commands::stats_commands::category_breakdown,
            commands::stats_commands::daily_summary,
            commands::stats_commands::get_imported_months,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
