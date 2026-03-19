use axum::Router;
use my_axum_api::{routes, store};

pub fn app() -> Router {
    routes::create_router(store::new_store())
}
