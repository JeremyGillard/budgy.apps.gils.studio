pub mod db;
pub mod models;
pub mod schema;

pub mod commands;
pub mod services;

use db::establish_connection;

pub fn run() {
    let conn = establish_connection();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(db::DbState(std::sync::Mutex::new(conn)))
        .invoke_handler(tauri::generate_handler![
            commands::transactions::list_transactions,
            commands::import::import_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
