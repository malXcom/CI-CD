use axum::{
    extract::{Path, Query, State, rejection::{PathRejection, JsonRejection}},
    http::StatusCode,
    Json,
};
use crate::{
    errors::AppError,
    models::{CreateStudent, SearchQuery, StudentStats, UpdateStudent, Student},
    services::students as service,
    store::SharedStore,
};

pub async fn list(State(store): State<SharedStore>) -> Json<Vec<Student>> {
    Json(service::get_all(&store))
}

pub async fn show(
    State(store): State<SharedStore>,
    id: Result<Path<u32>, PathRejection>,
) -> Result<Json<Student>, AppError> {
    let Path(id) = id.map_err(AppError::from)?;
    service::get_by_id(&store, id).map(Json)
}

pub async fn create(
    State(store): State<SharedStore>,
    payload: Result<Json<CreateStudent>, JsonRejection>,
) -> Result<(StatusCode, Json<Student>), AppError> {
    let Json(payload) = payload.map_err(AppError::from)?;
    service::create(&store, payload).map(|s| (StatusCode::CREATED, Json(s)))
}

pub async fn update(
    State(store): State<SharedStore>,
    id: Result<Path<u32>, PathRejection>,
    payload: Result<Json<UpdateStudent>, JsonRejection>,
) -> Result<Json<Student>, AppError> {
    let Path(id) = id.map_err(AppError::from)?;
    let Json(payload) = payload.map_err(AppError::from)?;
    service::update(&store, id, payload).map(Json)
}
pub async fn delete(
    State(store): State<SharedStore>,
    id: Result<Path<u32>, PathRejection>,
) -> Result<StatusCode, AppError> {
    let Path(id) = id.map_err(AppError::from)?;
    service::delete(&store, id).map(|_| StatusCode::NO_CONTENT)
}

pub async fn stats(
    State(store): State<SharedStore>,
) -> Result<Json<StudentStats>, AppError> {
    service::get_stats(&store).map(Json)
}

pub async fn search(
    State(store): State<SharedStore>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<Student>>, AppError> {
    service::search(&store, params.q).map(Json)
}
