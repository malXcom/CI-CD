use axum::Router;
use axum::routing::get;
use crate::{controllers::students, store::SharedStore};

pub fn create_router(store: SharedStore) -> Router {
    Router::new()
        .route("/students",         get(students::list).post(students::create))
        .route("/students/stats",   get(students::stats))
        .route("/students/search",  get(students::search))
        .route("/students/{id}",    get(students::show)
                                        .put(students::update)
                                        .delete(students::delete))
        .with_state(store)
}
