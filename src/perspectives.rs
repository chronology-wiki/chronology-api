use rocket_contrib::json::Json;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

use crate::models::*;

#[get("/api/perspectives")]
pub fn list() -> Json<Vec<Perspective>> {
  let perspectives: Vec<Perspective> = perspectives::table
    .select(perspectives::all_columns)
    .load::<Perspective>(&crate::establish_connection())
    .expect("Error retrieving perspectives");

  Json(perspectives)
}

#[get("/api/perspectives/<perspective_id>/events")]
pub fn perspective_events(perspective_id: i32) -> Json<Vec<PerspectiveEvent>> {
  let tuples: Vec<(Perspective, PerspectiveEvent)> = perspectives::table
    .inner_join(perspective_events::table)
    .filter(perspective_events::perspective_id.eq(perspective_id))
    .filter(perspective_events::is_latest.eq(true))
    .load::<(Perspective, PerspectiveEvent)>(&crate::establish_connection())
    .expect("Error retrieving perspectives");

  let mut events: Vec<PerspectiveEvent> = Vec::new();
  for tuple in tuples {
    events.push(tuple.1);
  }

  Json(events)
}