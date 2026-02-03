use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vaccine {
    pub id: String,
    pub name: String,
    pub dose_number: i32,
    pub min_age_days: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledVaccine {
    pub vaccine_id: String,
    pub vaccine_name: String,
    pub dose_number: i32,
    pub planned_date: String, // YYYY-MM-DD
    pub status: String,        // 'PENDING', 'COMPLETED', 'DELAYED', 'CANCELLED'
}
