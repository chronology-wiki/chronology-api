use rocket_contrib::json::Json;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

/* Database data structs (Hero, NewHero) */
use crate::models::*;

/* List our inserted heroes */
#[get("/")]
pub fn list() -> Json<Vec<Hero>> {
  /* Get all our heroes from database */
  let heroes: Vec<Hero> = heroes::table
    .select(heroes::all_columns)
    .load::<Hero>(&crate::establish_connection())
    .expect("Whoops, like this went bananas!");

  /* Return the json */
  Json(heroes)
}