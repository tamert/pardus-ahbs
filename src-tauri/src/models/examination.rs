use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Examination {
    pub id: Option<i64>,
    pub patient_id: i64,
    pub exam_date: String,
    pub complaint: Option<String>,
    pub findings: Option<String>,
    pub diagnosis: Option<String>,
    pub treatment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateExaminationInput {
    pub patient_id: i64,
    pub complaint: Option<String>,
    pub findings: Option<String>,
    pub diagnosis: Option<String>,
    pub treatment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Prescription {
    pub id: Option<i64>,
    pub exam_id: i64,
    pub medication_name: String,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePrescriptionInput {
    pub exam_id: i64,
    pub medication_name: String,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
}
