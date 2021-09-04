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

use rocket::http::Method;
use rocket::{routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

/* Declaring a module, just for separating things better */
pub mod topics;
pub mod events;
pub mod perspectives;

/* Will hold our data structs */
pub mod models;

/* auto-generated table macros */
pub mod schema;


/* This will return our pg connection to use with diesel */
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
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
        perspectives::create_perspective_event
        ]).attach(cors).launch();
}