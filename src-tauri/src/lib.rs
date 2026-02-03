pub mod commands;
pub mod models;

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tauri::{Manager, State};

pub struct DbState {
    pub pool: SqlitePool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Veritabanını arka planda asenkron olarak başlatıyoruz
            tauri::async_runtime::block_on(async move {
                let db_url = "sqlite:ahbs.db?mode=rwc"; 
                
                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(db_url)
                    .await
                    .expect("Veritabanına bağlanılamadı");

                // Tabloları oluştur (Migration)
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS patients (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL,
                        surname TEXT NOT NULL,
                        tc_no TEXT UNIQUE NOT NULL,
                        birth_date TEXT NOT NULL,
                        gender TEXT NOT NULL,
                        phone TEXT,
                        address TEXT
                    )"
                )
                .execute(&pool)
                .await
                .expect("Tablo oluşturulamadı");

                app.manage(DbState { pool });
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::patients::create_patient,
            commands::patients::get_patients,
            commands::patients::search_patient
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
