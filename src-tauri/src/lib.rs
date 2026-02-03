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
                .expect("Patients tablosu oluşturulamadı");

                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS examinations (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        patient_id INTEGER NOT NULL,
                        exam_date TEXT NOT NULL,
                        complaint TEXT,
                        findings TEXT,
                        diagnosis TEXT,
                        treatment TEXT,
                        FOREIGN KEY(patient_id) REFERENCES patients(id)
                    )"
                )
                .execute(&pool)
                .await
                .expect("Examinations tablosu oluşturulamadı");

                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS prescriptions (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        exam_id INTEGER NOT NULL,
                        medication_name TEXT NOT NULL,
                        dosage TEXT,
                        frequency TEXT,
                        FOREIGN KEY(exam_id) REFERENCES examinations(id)
                    )"
                )
                .execute(&pool)
                .await
                .expect("Prescriptions tablosu oluşturulamadı");

                // Aşı Tanımları Tablosu
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS vaccines (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        code TEXT UNIQUE NOT NULL,
                        name TEXT NOT NULL,
                        month_offset INTEGER NOT NULL,
                        description TEXT
                    )"
                )
                .execute(&pool)
                .await
                .expect("Vaccines tablosu oluşturulamadı");

                // Varsayılan aşıları ekle (Eğer yoksa)
                let vaccines_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vaccines")
                    .fetch_one(&pool)
                    .await
                    .unwrap_or((0,));
                
                if vaccines_count.0 == 0 {
                    sqlx::query(
                        "INSERT INTO vaccines (code, name, month_offset) VALUES 
                        ('HEPB1', 'Hepatit B 1. Doz', 0),
                        ('HEPB2', 'Hepatit B 2. Doz', 1),
                        ('HEPB3', 'Hepatit B 3. Doz', 6),
                        ('BCG', 'Verem (BCG)', 2),
                        ('KPA1', 'Zatürre (KPA) 1. Doz', 2),
                        ('KPA2', 'Zatürre (KPA) 2. Doz', 4),
                        ('KPA3', 'Zatürre (KPA) 3. Doz', 12),
                        ('KKK', 'Kızamık-Kızamıkçık-Kabakulak (KKK)', 12),
                        ('DABT1', 'Beşli Karma 1. Doz', 2),
                        ('DABT2', 'Beşli Karma 2. Doz', 4),
                        ('DABT3', 'Beşli Karma 3. Doz', 6),
                        ('DABT4', '48. Ay Pekiştirme aşıları', 48)"
                    )
                    .execute(&pool)
                    .await
                    .expect("Varsayılan aşılar eklenemedi");
                }

                // Hasta Aşı Takvimi Tablosu
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS patient_vaccinations_v2 (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        patient_id INTEGER NOT NULL,
                        vaccine_code TEXT NOT NULL,
                        vaccine_name TEXT NOT NULL,
                        scheduled_date TEXT NOT NULL,
                        administered_date TEXT,
                        status TEXT DEFAULT 'PENDING',
                        lot_no TEXT,
                        injection_site TEXT,
                        notes TEXT,
                        FOREIGN KEY(patient_id) REFERENCES patients(id)
                    )"
                )
                .execute(&pool)
                .await
                .expect("Patient Vaccinations tablosu oluşturulamadı");

                app.manage(DbState { pool });
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::patients::create_patient,
            commands::patients::get_patients,
            commands::patients::search_patient,
            commands::vaccination::get_vaccine_definitions,
            commands::vaccination::get_patient_vaccinations,
            commands::vaccination::initialize_patient_schedule,
            commands::vaccination::update_vaccination_status,
            commands::examinations::create_examination,
            commands::examinations::get_patient_examinations,
            commands::examinations::create_prescription,
            commands::examinations::get_examination_prescriptions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
