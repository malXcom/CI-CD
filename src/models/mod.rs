use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Field {
    Informatique,
    Mathematiques,
    Physique,
    Chimie,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub grade: f32,
    pub field: Field,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStudent {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub grade: f32,
    pub field: Field,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStudent {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub grade: Option<f32>,
    pub field: Option<Field>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentStats {
    pub total_students: i32,
    pub average_grade: f32,
    pub students_by_field: HashMap<String, i32>,
    pub best_student: Student,
}
