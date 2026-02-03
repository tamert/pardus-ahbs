use crate::models::vaccine::ScheduledVaccine;
use chrono::{Duration, NaiveDate};
use tauri::command;

#[command]
pub fn calculate_vaccination_schedule(birth_date_str: &str) -> Result<Vec<ScheduledVaccine>, String> {
    let birth_date = NaiveDate::parse_from_str(birth_date_str, "%Y-%m-%d")
        .map_err(|_| "Geçersiz doğum tarihi formatı. YYYY-MM-DD bekliyor.")?;

    let mut schedule = Vec::new();

    // Aşı Tanımları: (Ad, Doz, Gün Değişimi)
    let vaccine_definitions = vec![
        // Hepatit B
        ("Hepatit B", 1, 0),
        ("Hepatit B", 2, 30),
        ("Hepatit B", 3, 180),
        // BCG
        ("BCG", 1, 60),
        // DaBT-İPA-Hib (Beşli Karma)
        ("DaBT-İPA-Hib", 1, 60),
        ("DaBT-İPA-Hib", 2, 120),
        ("DaBT-İPA-Hib", 3, 180),
        ("DaBT-İPA-Hib", 4, 540),
        // KKK (Kızamık, Kızamıkçık, Kabakulak)
        ("KKK", 1, 365),
        ("KKK", 2, 1460), // 4 yaş
        // PPA (Pnomokok)
        ("KPA", 1, 60),
        ("KPA", 2, 120),
        ("KPA", 3, 365),
    ];

    for (name, dose, days) in vaccine_definitions {
        let planned_date = birth_date + Duration::days(days as i64);
        schedule.push(ScheduledVaccine {
            vaccine_id: name.to_lowercase().replace("-", "_"),
            vaccine_name: name.to_string(),
            dose_number: dose,
            planned_date: planned_date.format("%Y-%m-%d").to_string(),
            status: "PENDING".to_string(),
        });
    }

    // Tarihe göre sırala
    schedule.sort_by(|a, b| a.planned_date.cmp(&b.planned_date));

    Ok(schedule)
}
