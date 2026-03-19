#![deny(unused_variables)]
#![deny(unused_imports)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]

pub mod controllers;
pub mod errors;
pub mod models;
pub mod routes;
pub mod services;
pub mod store;
