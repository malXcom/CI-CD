use std::collections::HashMap;
use crate::{
    errors::AppError,
    models::{CreateStudent, Student, StudentStats, UpdateStudent},
    store::SharedStore,
};

pub fn validate_create(payload: &CreateStudent) -> Result<(), AppError> {
    if payload.first_name.trim().len() < 2 {
        return Err(AppError::Validation(
            "firstName must be at least 2 characters".into(),
        ));
    }
    if payload.last_name.trim().len() < 2 {
        return Err(AppError::Validation(
            "lastName must be at least 2 characters".into(),
        ));
    }
    if !payload.email.contains('@') || !payload.email.contains('.') {
        return Err(AppError::Validation("invalid email".into()));
    }
    if !(0.0..=20.0).contains(&payload.grade) {
        return Err(AppError::Validation("grade must be between 0 and 20".into()));
    }
    Ok(())
}

pub fn validate_update(payload: &UpdateStudent) -> Result<(), AppError> {
    if let Some(ref v) = payload.first_name {
        if v.trim().len() < 2 {
            return Err(AppError::Validation(
                "firstName must be at least 2 characters".into(),
            ));
        }
    }
    if let Some(ref v) = payload.last_name {
        if v.trim().len() < 2 {
            return Err(AppError::Validation(
                "lastName must be at least 2 characters".into(),
            ));
        }
    }
    if let Some(ref v) = payload.email {
        if !v.contains('@') || !v.contains('.') {
            return Err(AppError::Validation("invalid email".into()));
        }
    }
    if let Some(grade) = payload.grade {
        if !(0.0..=20.0).contains(&grade) {
            return Err(AppError::Validation("grade must be between 0 and 20".into()));
        }
    }
    Ok(())
}

pub fn get_all(store: &SharedStore) -> Vec<Student> {
    store.lock().unwrap().clone()
}

pub fn get_by_id(store: &SharedStore, id: u32) -> Result<Student, AppError> {
    store
        .lock()
        .unwrap()
        .iter()
        .find(|s| s.id == id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("student {id} not found")))
}

pub fn create(store: &SharedStore, payload: CreateStudent) -> Result<Student, AppError> {
    validate_create(&payload)?;

    let mut store = store.lock().unwrap();

    if store.iter().any(|s| s.email.to_lowercase() == payload.email.to_lowercase()) {
        return Err(AppError::Conflict(format!(
            "email {} already in use",
            payload.email
        )));
    }

    let next_id = store.iter().map(|s| s.id).max().unwrap_or(0) + 1;
    let student = Student {
        id: next_id,
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        grade: payload.grade,
        field: payload.field,
    };
    store.push(student.clone());
    Ok(student)
}

pub fn update(
    store: &SharedStore,
    id: u32,
    payload: UpdateStudent,
) -> Result<Student, AppError> {
    validate_update(&payload)?;

    let mut store = store.lock().unwrap();

    if let Some(ref new_email) = payload.email {
        if store.iter().any(|s| s.id != id && &s.email == new_email) {
            return Err(AppError::Conflict(format!(
                "email {} already in use",
                new_email
            )));
        }
    }

    let student = store
        .iter_mut()
        .find(|s| s.id == id)
        .ok_or_else(|| AppError::NotFound(format!("student {id} not found")))?;

    if let Some(v) = payload.first_name { student.first_name = v; }
    if let Some(v) = payload.last_name  { student.last_name = v; }
    if let Some(v) = payload.email      { student.email = v; }
    if let Some(v) = payload.grade      { student.grade = v; }
    if let Some(v) = payload.field      { student.field = v; }

    Ok(student.clone())
}

pub fn delete(store: &SharedStore, id: u32) -> Result<(), AppError> {
    let mut store = store.lock().unwrap();
    let initial_len = store.len();
    store.retain(|s| s.id != id);
    if store.len() == initial_len {
        return Err(AppError::NotFound(format!("student {id} not found")));
    }
    Ok(())
}

pub fn get_stats(store: &SharedStore) -> Result<StudentStats, AppError> {
    let store = store.lock().unwrap();

    if store.is_empty() {
        return Err(AppError::NotFound("no students registered".into()));
    }

    let total_students = store.len() as i32;
    let average_grade = store.iter().map(|s| s.grade).sum::<f32>() / total_students as f32;
    let best_student = store
        .iter()
        .max_by(|a, b| a.grade.partial_cmp(&b.grade).unwrap())
        .cloned()
        .unwrap();

    let mut students_by_field: HashMap<String, i32> = HashMap::new();
    for student in store.iter() {
        let key = format!("{:?}", student.field).to_lowercase();
        *students_by_field.entry(key).or_insert(0) += 1;
    }

    Ok(StudentStats {
        total_students,
        average_grade,
        students_by_field,
        best_student,
    })
}

pub fn search(store: &SharedStore, q: Option<String>) -> Result<Vec<Student>, AppError> {
    let query = match q {
        Some(ref v) if v.trim().len() >= 2 => v.to_lowercase(),
        Some(_) => {
            return Err(AppError::Validation(
                "query must be at least 2 characters".into(),
            ))
        }
        None => return Ok(store.lock().unwrap().clone()),
    };

    let store = store.lock().unwrap();
    let results: Vec<Student> = store
        .iter()
        .filter(|s| {
            s.first_name.to_lowercase().contains(&query)
                || s.last_name.to_lowercase().contains(&query)
                || s.email.to_lowercase().contains(&query)
                || format!("{:?}", s.field).to_lowercase().contains(&query)
                || format!("{:?}", s.grade).to_lowercase().contains(&query)
        })
        .cloned()
        .collect();

    if results.is_empty() {
        return Err(AppError::NotFound(format!(
            "no students found for \"{}\"",
            q.unwrap()
        )));
    }

    Ok(results)
}
