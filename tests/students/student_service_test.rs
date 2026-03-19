mod common {
    include!("../common.rs");
}

use my_axum_api::{
    models::{CreateStudent, Field, UpdateStudent},
    services::students::{
        create, delete, get_all, get_by_id, get_stats, search, update, validate_create,
        validate_update,
    },
    store::{empty_store, new_store},
};

fn make_payload(email: &str, grade: f32) -> CreateStudent {
    CreateStudent {
        first_name: "Jean".to_string(),
        last_name: "Dupont".to_string(),
        email: email.to_string(),
        grade,
        field: Field::Informatique,
    }
}

#[test]
fn test_validate_first_name_too_short() {
    let mut p = make_payload("a@b.com", 10.0);
    p.first_name = "A".to_string();
    assert!(validate_create(&p).is_err());
}

#[test]
fn test_validate_last_name_too_short() {
    let mut p = make_payload("a@b.com", 10.0);
    p.last_name = "B".to_string();
    assert!(validate_create(&p).is_err());
}

#[test]
fn test_validate_invalid_email_no_at() {
    let p = make_payload("invalidemail.com", 10.0);
    assert!(validate_create(&p).is_err());
}

#[test]
fn test_validate_invalid_email_no_dot() {
    let p = make_payload("invalid@emailcom", 10.0);
    assert!(validate_create(&p).is_err());
}

#[test]
fn test_validate_grade_above_max() {
    let p = make_payload("a@b.com", 21.0);
    assert!(validate_create(&p).is_err());
}

#[test]
fn test_validate_grade_below_min() {
    let p = make_payload("a@b.com", -1.0);
    assert!(validate_create(&p).is_err());
}

#[test]
fn test_validate_grade_boundary_values() {
    assert!(validate_create(&make_payload("a@b.com", 0.0)).is_ok());
    assert!(validate_create(&make_payload("a@b.com", 20.0)).is_ok());
    assert!(validate_create(&make_payload("a@b.com", 10.5)).is_ok());
}

#[test]
fn test_validate_create_valid() {
    assert!(validate_create(&make_payload("valid@email.com", 15.0)).is_ok());
}

#[test]
fn test_validate_update_short_first_name() {
    let p = UpdateStudent {
        first_name: Some("A".to_string()),
        last_name: None,
        email: None,
        grade: None,
        field: None,
    };
    assert!(validate_update(&p).is_err());
}

#[test]
fn test_validate_update_invalid_grade() {
    let p = UpdateStudent {
        first_name: None,
        last_name: None,
        email: None,
        grade: Some(25.0),
        field: None,
    };
    assert!(validate_update(&p).is_err());
}

#[test]
fn test_validate_update_all_none_is_valid() {
    let p = UpdateStudent {
        first_name: None,
        last_name: None,
        email: None,
        grade: None,
        field: None,
    };
    assert!(validate_update(&p).is_ok());
}

#[test]
fn test_get_all_returns_seed_data() {
    let store = new_store();
    let students = get_all(&store);
    assert_eq!(students.len(), 2);
}

#[test]
fn test_get_by_id_found() {
    let store = new_store();
    let student = get_by_id(&store, 1);
    assert!(student.is_ok());
    assert_eq!(student.unwrap().id, 1);
}

#[test]
fn test_get_by_id_not_found() {
    let store = new_store();
    assert!(get_by_id(&store, 999).is_err());
}

#[test]
fn test_create_assigns_incremented_id() {
    let store = new_store();
    let student = create(&store, make_payload("new@email.com", 15.0)).unwrap();
    assert_eq!(student.id, 3);
}

#[test]
fn test_create_duplicate_email_fails() {
    let store = new_store();
    let result = create(&store, make_payload("john@example.com", 15.0));
    assert!(result.is_err());
}

#[test]
fn test_create_case_insensitive_email_conflict() {
    let store = new_store();
    let result = create(&store, make_payload("JOHN@EXAMPLE.COM", 15.0));
    assert!(result.is_err());
}

#[test]
fn test_update_grade_only() {
    let store = new_store();
    let payload = UpdateStudent {
        first_name: None,
        last_name: None,
        email: None,
        grade: Some(19.0),
        field: None,
    };
    let updated = update(&store, 1, payload).unwrap();
    assert_eq!(updated.grade, 19.0);
    assert_eq!(updated.id, 1);
}

#[test]
fn test_update_nonexistent_student() {
    let store = new_store();
    let payload = UpdateStudent {
        first_name: None,
        last_name: None,
        email: None,
        grade: Some(10.0),
        field: None,
    };
    assert!(update(&store, 999, payload).is_err());
}

#[test]
fn test_update_email_conflict_with_other_student() {
    let store = new_store();
    // use the actual email of student 2 from new_store()
    let payload = UpdateStudent {
        first_name: None,
        last_name: None,
        email: Some("bob@example.com".to_string()), // match your seed data
        grade: None,
        field: None,
    };
    assert!(update(&store, 1, payload).is_err());
}

#[test]
fn test_delete_existing_student() {
    let store = new_store();
    assert!(delete(&store, 1).is_ok());
    assert!(get_by_id(&store, 1).is_err());
}

#[test]
fn test_delete_nonexistent_student() {
    let store = new_store();
    assert!(delete(&store, 999).is_err());
}

#[test]
fn test_stats_total_students() {
    let store = new_store();
    let stats = get_stats(&store).unwrap();
    assert_eq!(stats.total_students, 2);
}

#[test]
fn test_stats_average_grade() {
    let store = new_store();
    let stats = get_stats(&store).unwrap();
    assert!(stats.average_grade > 0.0);
}

#[test]
fn test_stats_best_student_has_highest_grade() {
    let store = new_store();
    let stats = get_stats(&store).unwrap();
    let all = get_all(&store);
    let max_grade = all.iter().map(|s| s.grade).fold(f32::MIN, f32::max);
    assert_eq!(stats.best_student.grade, max_grade);
}

#[test]
fn test_stats_empty_store_returns_error() {
    let store = empty_store();
    assert!(get_stats(&store).is_err());
}

#[test]
fn test_search_by_first_name() {
    let store = new_store();
    let results = search(&store, Some("john".to_string())).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn test_search_no_match_returns_error() {
    let store = new_store();
    assert!(search(&store, Some("zzznomatch".to_string())).is_err());
}

#[test]
fn test_search_query_too_short_returns_error() {
    let store = new_store();
    assert!(search(&store, Some("a".to_string())).is_err());
}

#[test]
fn test_search_none_returns_all() {
    let store = new_store();
    let results = search(&store, None).unwrap();
    assert_eq!(results.len(), 2);
}
