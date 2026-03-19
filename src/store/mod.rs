use crate::models::Student;
use std::sync::{Arc, Mutex};

pub type SharedStore = Arc<Mutex<Vec<Student>>>;

pub fn empty_store() -> SharedStore {
    Arc::new(Mutex::new(vec![]))
}

pub fn new_store() -> SharedStore {
    Arc::new(Mutex::new(vec![
        Student {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@example.com".to_string(),
            grade: 17.5,
            field: crate::models::Field::Informatique,
        },
        Student {
            id: 2,
            first_name: "Bob".to_string(),
            last_name: "Martin".to_string(),
            email: "bob@example.com".to_string(),
            grade: 12.0,
            field: crate::models::Field::Mathematiques,
        },
    ]))
}
