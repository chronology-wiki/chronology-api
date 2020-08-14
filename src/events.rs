use rocket_contrib::json::Json;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

use crate::models::*;

#[get("/api/topics/<topic_id>/events")]
pub fn list(topic_id: i32) -> Json<Vec<Event>> {
  let event_tuples: Vec<(Event, TopicEvent)> = events::table
    .inner_join(topic_events::table)
    .filter(topic_events::topic_id.eq(topic_id))
    .load::<(Event, TopicEvent)>(&crate::establish_connection())
    .expect("Could not retrieve events");

  let mut events: Vec<Event> = Vec::new();
  for event_tuple in event_tuples {
    events.push(event_tuple.0);
  }

  Json(events)
}