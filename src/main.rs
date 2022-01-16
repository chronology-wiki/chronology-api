#![feature(proc_macro_hygiene, decl_macro)]

/* Our extern crates */
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

extern crate dotenv;

/* Importing functions */
use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

use diesel_logger::LoggingConnection;

use rocket::http::Method;
use rocket::{routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

/* Declaring a module, just for separating things better */
pub mod topics;
pub mod events;
pub mod perspectives;
pub mod merge_proposals;

/* Will hold our data structs */
pub mod models;

/* auto-generated table macros */
pub mod schema;


/* This will return our pg connection to use with diesel */
pub fn establish_connection() -> LoggingConnection<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    LoggingConnection::<PgConnection>::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    std::env::set_var("RUST_LOG", "diesel_logger=debug");
    env_logger::init();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Patch, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::default(),
        allow_credentials: false,
        ..Default::default()
    }
    .to_cors().unwrap();

    rocket::ignite().mount("/", routes![
        topics::list, 
        topics::get_topic,
        events::create,
        events::list,
        perspectives::list,
        perspectives::get_perspective_events_endpoint,
        perspectives::create_perspective_event,
        merge_proposals::get_merge_proposals,
        merge_proposals::get_user_merge_proposals,
        merge_proposals::create_merge_proposal
        ]).attach(cors).launch();
}