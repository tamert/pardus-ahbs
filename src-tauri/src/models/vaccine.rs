use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct VaccineDefinition {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub month_offset: i32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PatientVaccination {
    pub id: i64,
    pub patient_id: i64,
    pub vaccine_code: String,
    pub vaccine_name: String,
    pub scheduled_date: String,
    pub administered_date: Option<String>,
    pub status: String, // PENDING, COMPLETED, MISSED
    pub lot_no: Option<String>,
    pub injection_site: Option<String>,
    pub notes: Option<String>,
}

// For frontend convenience if we need a combined view
#[derive(Debug, Serialize, Deserialize)]
pub struct VaccineScheduleItem {
    pub id: i64,
    pub vaccine_code: String,
    pub vaccine_name: String,
    pub scheduled_date: String,
    pub status: String,
    pub is_overdue: bool
}
