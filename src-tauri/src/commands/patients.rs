use crate::models::patient::{CreatePatientInput, Patient};
use crate::DbState;
use tauri::State;

#[tauri::command]
pub async fn create_patient(
    input: CreatePatientInput,
    state: State<'_, DbState>,
) -> Result<i64, String> {
    let pool = &state.pool;

    let result = sqlx::query(
        "INSERT INTO patients (name, surname, tc_no, birth_date, gender, phone, address) 
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&input.name)
    .bind(&input.surname)
    .bind(&input.tc_no)
    .bind(&input.birth_date)
    .bind(&input.gender)
    .bind(&input.phone)
    .bind(&input.address)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.last_insert_rowid())
}

#[tauri::command]
pub async fn get_patients(state: State<'_, DbState>) -> Result<Vec<Patient>, String> {
    let pool = &state.pool;

    let patients = sqlx::query_as::<_, Patient>("SELECT * FROM patients ORDER BY id DESC")
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(patients)
}

#[tauri::command]
pub async fn search_patient(query: String, state: State<'_, DbState>) -> Result<Vec<Patient>, String> {
    let pool = &state.pool;
    let search_query = format!("%{}%", query);

    let patients = sqlx::query_as::<_, Patient>(
        "SELECT * FROM patients WHERE name LIKE ? OR surname LIKE ? OR tc_no LIKE ? LIMIT 20"
    )
    .bind(&search_query)
    .bind(&search_query)
    .bind(&search_query)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(patients)
}
