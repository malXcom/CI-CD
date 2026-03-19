use axum::Router;
use my_axum_api::{routes, store};

#[allow(dead_code)]
pub fn app() -> Router {
    routes::create_router(store::new_store())
}
