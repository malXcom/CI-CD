#![deny(unused_variables)]
#![deny(unused_imports)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]

use my_axum_api::{routes, store};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let store = store::new_store();
    let app = routes::create_router(store);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Serveur is running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
