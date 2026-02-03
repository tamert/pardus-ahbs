use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tauri::{AppHandle, Manager, State};

struct DbState {
    pool: SqlitePool,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(name: &str, state: State<'_, DbState>) -> Result<String, String> {
    // Burada basit bir SQL sorgusu yaparak DB'nin çalıştığını test edebiliriz
    // sqlx::query("SELECT 1").execute(&state.pool).await.map_err(|e| e.to_string())?;
    
    Ok(format!("Merhaba, {}! AHBS Veritabanı bağlantısı aktif.", name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            
            // Veritabanını arka planda asenkron olarak başlatıyoruz
            tauri::async_runtime::block_on(async move {
                let db_url = "sqlite:ahbs.db?mode=rwc"; // App root'ta bir dosya oluşturur
                
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
                        tc_no TEXT UNIQUE,
                        birth_date TEXT
                    )"
                )
                .execute(&pool)
                .await
                .expect("Tablo oluşturulamadı");

                app.manage(DbState { pool });
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
