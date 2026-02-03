use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Patient {
    pub id: Option<i64>,
    pub name: String,
    pub surname: String,
    pub tc_no: String,
    pub birth_date: String,
    pub gender: String, // 'E' or 'K'
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePatientInput {
    pub name: String,
    pub surname: String,
    pub tc_no: String,
    pub birth_date: String,
    pub gender: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}
