use crate::DbState;
use crate::models::vaccine::{VaccineDefinition, PatientVaccination};
use chrono::{NaiveDate, Datelike};
use tauri::{State, command};

#[command]
pub async fn get_vaccine_definitions(state: State<'_, DbState>) -> Result<Vec<VaccineDefinition>, String> {
    let vaccines = sqlx::query_as::<_, VaccineDefinition>("SELECT * FROM vaccines ORDER BY month_offset")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(vaccines)
}

#[command]
pub async fn get_patient_vaccinations(
    patient_id: i64,
    state: State<'_, DbState>
) -> Result<Vec<PatientVaccination>, String> {
    let vaccinations = sqlx::query_as::<_, PatientVaccination>(
        "SELECT * FROM patient_vaccinations_v2 WHERE patient_id = ? ORDER BY scheduled_date"
    )
    .bind(patient_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(vaccinations)
}

#[command]
pub async fn initialize_patient_schedule(
    patient_id: i64,
    birth_date_str: String,
    state: State<'_, DbState>
) -> Result<Vec<PatientVaccination>, String> {
    // 1. Check if schedule already exists
    let existing = get_patient_vaccinations(patient_id, state.clone()).await?;
    if !existing.is_empty() {
        return Ok(existing);
    }

    // 2. Get definitions
    let definitions = get_vaccine_definitions(state.clone()).await?;
    let birth_date = NaiveDate::parse_from_str(&birth_date_str, "%Y-%m-%d")
        .map_err(|_| "Geçersiz doğum tarihi formatı")?;

    // 3. Generate and Insert
    for def in definitions {
        // Simple month addition logic (approximate 30 days per month for safety if chrono logic is complex, 
        // but let's try to use months logic correctly if possible, otherwise fallback to days)
        // Since sqlite doesn't have interval math here, we calculate in Rust.
        
        let target_date = add_months(birth_date, def.month_offset as u32);
        let scheduled_date_str = target_date.format("%Y-%m-%d").to_string();

        sqlx::query(
            "INSERT INTO patient_vaccinations_v2 (patient_id, vaccine_code, vaccine_name, scheduled_date) VALUES (?, ?, ?, ?)"
        )
        .bind(patient_id)
        .bind(&def.code)
        .bind(&def.name)
        .bind(&scheduled_date_str)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    // 4. Return new list
    get_patient_vaccinations(patient_id, state).await
}

#[command]
pub async fn update_vaccination_status(
    id: i64,
    status: String,
    administered_date: Option<String>,
    lot_no: Option<String>,
    injection_site: Option<String>,
    state: State<'_, DbState>
) -> Result<(), String> {
    sqlx::query(
        "UPDATE patient_vaccinations_v2 SET status = ?, administered_date = ?, lot_no = ?, injection_site = ? WHERE id = ?"
    )
    .bind(status)
    .bind(administered_date)
    .bind(lot_no)
    .bind(injection_site)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

// Helper for month addition
fn add_months(date: NaiveDate, months: u32) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month();
    let day = date.day();

    month += months;
    while month > 12 {
        month -= 12;
        year += 1;
    }

    // Handle day overflow (e.g. Feb 30 -> Feb 28)
    let last_day_of_month = get_days_from_month(year, month);
    let day = std::cmp::min(day, last_day_of_month);

    NaiveDate::from_ymd_opt(year, month, day).unwrap_or(date)
}

fn get_days_from_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(
        if month == 12 { year + 1 } else { year },
        if month == 12 { 1 } else { month + 1 },
        1,
    )
    .unwrap()
    .pred_opt()
    .unwrap()
    .day()
}
