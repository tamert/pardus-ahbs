use tauri::State;
use crate::DbState;
use crate::models::examination::{Examination, CreateExaminationInput, Prescription, CreatePrescriptionInput};
use chrono::Local;

#[tauri::command]
pub async fn create_examination(
    input: CreateExaminationInput,
    state: State<'_, DbState>,
) -> Result<i64, String> {
    let pool = &state.pool;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let result = sqlx::query(
        "INSERT INTO examinations (patient_id, exam_date, complaint, findings, diagnosis, treatment) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(input.patient_id)
    .bind(now)
    .bind(&input.complaint)
    .bind(&input.findings)
    .bind(&input.diagnosis)
    .bind(&input.treatment)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.last_insert_rowid())
}

#[tauri::command]
pub async fn get_patient_examinations(
    patient_id: i64,
    state: State<'_, DbState>,
) -> Result<Vec<Examination>, String> {
    let pool = &state.pool;

    let examinations = sqlx::query_as::<_, Examination>(
        "SELECT * FROM examinations WHERE patient_id = ? ORDER BY exam_date DESC"
    )
    .bind(patient_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(examinations)
}

#[tauri::command]
pub async fn create_prescription(
    input: CreatePrescriptionInput,
    state: State<'_, DbState>,
) -> Result<i64, String> {
    let pool = &state.pool;

    let result = sqlx::query(
        "INSERT INTO prescriptions (exam_id, medication_name, dosage, frequency) 
         VALUES (?, ?, ?, ?)"
    )
    .bind(input.exam_id)
    .bind(&input.medication_name)
    .bind(&input.dosage)
    .bind(&input.frequency)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.last_insert_rowid())
}

#[tauri::command]
pub async fn get_examination_prescriptions(
    exam_id: i64,
    state: State<'_, DbState>,
) -> Result<Vec<Prescription>, String> {
    let pool = &state.pool;

    let prescriptions = sqlx::query_as::<_, Prescription>(
        "SELECT * FROM prescriptions WHERE exam_id = ?"
    )
    .bind(exam_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(prescriptions)
}
