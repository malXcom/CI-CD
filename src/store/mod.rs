use crate::models::{Field, Student};
use std::sync::{Arc, Mutex};

pub type SharedStore = Arc<Mutex<Vec<Student>>>;

pub fn new_store() -> SharedStore {
    Arc::new(Mutex::new(vec![
        Student {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@example.com".to_string(),
            grade: 17.5,
            field: Field::Informatique,
        },
        Student {
            id: 2,
            first_name: "Bob".to_string(),
            last_name: "Martin".to_string(),
            email: "bob@example.com".to_string(),
            grade: 14.5,
            field: Field::Physique,
        },
        Student {
            id: 3,
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            email: "jane@example.com".to_string(),
            grade: 12.0,
            field: Field::Mathematiques,
        },
        Student {
            id: 4,
            first_name: "Alice".to_string(),
            last_name: "Dupont".to_string(),
            email: "alice@example.com".to_string(),
            grade: 18.0,
            field: Field::Chimie,
        },
        Student {
            id: 5,
            first_name: "Charles".to_string(),
            last_name: "Bernard".to_string(),
            email: "charles@example.com".to_string(),
            grade: 11.5,
            field: Field::Informatique,
        },
        Student {
            id: 6,
            first_name: "Sophie".to_string(),
            last_name: "Leclerc".to_string(),
            email: "sophie@example.com".to_string(),
            grade: 15.0,
            field: Field::Mathematiques,
        },
        Student {
            id: 7,
            first_name: "Lucas".to_string(),
            last_name: "Moreau".to_string(),
            email: "lucas@example.com".to_string(),
            grade: 9.5,
            field: Field::Physique,
        },
        Student {
            id: 8,
            first_name: "Emma".to_string(),
            last_name: "Petit".to_string(),
            email: "emma@example.com".to_string(),
            grade: 19.0,
            field: Field::Chimie,
        },
        Student {
            id: 9,
            first_name: "Hugo".to_string(),
            last_name: "Roux".to_string(),
            email: "hugo@example.com".to_string(),
            grade: 13.0,
            field: Field::Informatique,
        },
        Student {
            id: 10,
            first_name: "Camille".to_string(),
            last_name: "Fournier".to_string(),
            email: "camille@example.com".to_string(),
            grade: 16.5,
            field: Field::Mathematiques,
        },
        Student {
            id: 11,
            first_name: "Nathan".to_string(),
            last_name: "Girard".to_string(),
            email: "nathan@example.com".to_string(),
            grade: 10.0,
            field: Field::Physique,
        },
        Student {
            id: 12,
            first_name: "Léa".to_string(),
            last_name: "Bonnet".to_string(),
            email: "lea@example.com".to_string(),
            grade: 17.0,
            field: Field::Chimie,
        },
        Student {
            id: 13,
            first_name: "Thomas".to_string(),
            last_name: "Chevalier".to_string(),
            email: "thomas@example.com".to_string(),
            grade: 8.5,
            field: Field::Informatique,
        },
        Student {
            id: 14,
            first_name: "Inès".to_string(),
            last_name: "Gauthier".to_string(),
            email: "ines@example.com".to_string(),
            grade: 14.0,
            field: Field::Mathematiques,
        },
        Student {
            id: 15,
            first_name: "Maxime".to_string(),
            last_name: "Robin".to_string(),
            email: "maxime@example.com".to_string(),
            grade: 20.0,
            field: Field::Physique,
        },
    ]))
}

pub fn empty_store() -> SharedStore {
    Arc::new(Mutex::new(vec![]))
}