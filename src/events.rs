use rocket_contrib::json::Json;
use std::collections::HashMap;
use std::collections::HashSet;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

use crate::models::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct EventWithPerspectives {
  event: Event,
  perspectives: HashMap<i32, PerspectiveEvent>
}

#[get("/api/topics/<topic_id>/events?<perspectives>")]
pub fn list(topic_id: i32, perspectives: Option<String>) -> Json<Vec<EventWithPerspectives>> {
  let perspective_ids_str = perspectives.unwrap_or(String::default());
  let ids_iter = perspective_ids_str.split(",");
  let mut perspective_ids: Vec<i32> = Vec::new();

  for id_str in ids_iter {
    let id = String::from(id_str).parse::<i32>().unwrap_or(0);
    perspective_ids.push(id);
  }

  let event_tuples: Vec<(Event, TopicEvent, Option<PerspectiveEvent>)> = events::table
    .inner_join(topic_events::table)
    .filter(topic_events::topic_id.eq(topic_id))
    .left_join(perspective_events::table)
    .filter(perspective_events::perspective_id.eq_any(perspective_ids))
    .or_filter(perspective_events::perspective_id.is_null())
    .load::<(Event, TopicEvent, Option<PerspectiveEvent>)>(&crate::establish_connection())
    .expect("Could not retrieve events");

  let mut events_with_perspectives = HashMap::<i32, EventWithPerspectives>::new();
  let mut event_ids = HashSet::<i32>::new();

  for event_tuple in event_tuples {
    let event = event_tuple.0;
    let event_id = event.event_id;
    event_ids.insert(event_id);

    let perspective_event_option: Option<PerspectiveEvent> = event_tuple.2;
    if perspective_event_option.is_some() {
      let persp_evt = perspective_event_option.unwrap();
      if !events_with_perspectives.get(&event_id).is_some() {
        events_with_perspectives.insert(event_id, EventWithPerspectives {
          event: event,
          perspectives: HashMap::new()
        });
      }
      events_with_perspectives.get_mut(&event_id).unwrap().perspectives.insert(persp_evt.perspective_id, persp_evt);
    }

  }
  
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

  Json(result)
}
