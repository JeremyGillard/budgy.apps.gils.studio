pub mod commands;
pub mod csv;
pub mod db;
pub mod error;
pub mod models;
pub mod services;

use std::sync::Mutex;

use commands::auth_commands::DbPath;
use commands::import_commands::DbConn;
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

            app.manage(DbConn(Mutex::new(None)));
            app.manage(DbPath(db_url));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth_commands::get_db_status,
            commands::auth_commands::unlock_db,
            commands::auth_commands::setup_encryption,
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
            commands::stats_commands::yearly_earnings,
            commands::stats_commands::yearly_expenses,
            commands::stats_commands::avg_monthly_spend,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
