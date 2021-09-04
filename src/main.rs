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
    let allowed_origins = AllowedOrigins::All;
    let cors_result = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::default(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors();

    let cors = cors_result.unwrap();

    rocket::ignite().mount("/", routes![
        topics::list, 
        topics::get_topic,
        events::list,
        perspectives::list,
        perspectives::perspective_events_endpoint
        ])
        .attach(cors)
        .launch();
}