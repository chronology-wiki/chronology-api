use rocket_contrib::json::Json;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

use serde::Deserialize;

use crate::models::*;

#[get("/api/perspectives")]
pub fn list() -> Json<Vec<Perspective>> {
  let perspectives: Vec<Perspective> = perspectives::table
    .select(perspectives::all_columns)
    .load::<Perspective>(&crate::establish_connection())
    .expect("Error retrieving perspectives");

  Json(perspectives)
}

#[derive(Deserialize)]
pub struct CreatePerspectiveEventReqBody {
  event_id: i32,
  name: Option<String>,
  description: Option<String>,
  historicity_stance: Historicity_stance_enum,
  relevance_stance: i32,
}

#[derive(Insertable)]
#[table_name = "perspective_events"]
struct PerspectiveEventToCreate {
  event_id: i32,
  name: Option<String>,
  description: Option<String>,
  historicity_stance: Historicity_stance_enum,
  relevance_stance: i32,
  perspective_id: i32,
  created_by: i32
}

#[post("/api/perspectives/<perspective_id>/events", data = "<request_body>")]
pub fn create_perspective_event(perspective_id: i32, request_body: Json<CreatePerspectiveEventReqBody>) -> Json<PerspectiveEvent> {
  let req: CreatePerspectiveEventReqBody = request_body.into_inner();

  let evt_to_insert = PerspectiveEventToCreate {
    event_id: req.event_id,
    name: req.name,
    description: req.description,
    historicity_stance: req.historicity_stance,
    relevance_stance: req.relevance_stance,
    perspective_id: perspective_id,
    created_by: 1
  };

  let db_res: Vec<PerspectiveEvent> = diesel::insert_into(perspective_events::table)
    .values(evt_to_insert)
    .load::<PerspectiveEvent>(&crate::establish_connection())
    .expect("Could not insert perspective event");

  Json(db_res.into_iter().nth(0).unwrap())
}

#[get("/api/perspectives/<perspective_id>/events")]
pub fn get_perspective_events_endpoint(perspective_id: i32) -> Json<Vec<PerspectiveEvent>> {
  let events = perspective_events(perspective_id);

  Json(events)
}

pub fn perspective_events(perspective_id: i32) -> Vec<PerspectiveEvent> {
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

  events
}