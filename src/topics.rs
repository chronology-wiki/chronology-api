use rocket_contrib::json::Json;

use diesel::prelude::*;
use crate::schema::*;
use crate::models::*;
use crate::events::get_perspective_ids;
use crate::events::get_perspective_events;
use crate::events::EventWithPerspectives;
use crate::endpoint_utils::RequestError;
use crate::endpoint_utils::RequestErrorExplanation;
use std::collections::HashMap;
use std::collections::HashSet;

#[get("/api/topics")]
pub fn list() -> Json<Vec<Topic>> {
  let topics: Vec<Topic> = topics::table
    .select(topics::all_columns)
    .load::<Topic>(&crate::establish_connection())
    .expect("Error retrieving topics");

  Json(topics)
}

#[get("/api/topics/<topic_id>")]
pub fn get_topic(topic_id: i32) -> Json<Topic> {
  let topic: Topic = topics::table
    .find(topic_id)
    .first::<Topic>(&crate::establish_connection())
    .expect("Error retrieving topic");

  Json(topic)
}

#[get("/api/topics/<topic_id>/perspective-diffs?<perspective>")]
pub fn get_topic_perspective_diffs(topic_id: i32, perspective: Option<String>) -> Result<Json<Vec<EventWithPerspectives>>, RequestError> {
  let perspective_ids = get_perspective_ids(perspective);

  if perspective_ids.len() != 2 {
    Err(RequestError::BadRequest(Json(RequestErrorExplanation {
      error: "Two valid perspective ids must be provided as comma separated values of the perspective query param"
    })))
  }
  else {
    let (event_ids, mut events_with_perspectives): (HashSet::<i32>, HashMap::<i32, EventWithPerspectives>) = get_perspective_events(topic_id, perspective_ids);

    let mut result = Vec::new();

    for event_id in event_ids {
      match events_with_perspectives.remove(&event_id) {
        Some(event_with_persp) => {
          result.push(event_with_persp);
        },
        None => {
        }
      }
    }

    Ok(Json(result))
  }
}