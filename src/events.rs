use rocket_contrib::json::Json;
use std::collections::HashMap;
use std::collections::HashSet;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

use crate::models::*;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Debug)]
pub struct EventWithPerspectives {
  event: Event,
  perspectives: HashMap<i32, PerspectiveEvent>
}

#[derive(Deserialize)]
pub struct CreateEventRequestBody {
  name: String,
  description: Option<String>,
  url_slug: String,
  topics: Vec<i32>
}

#[derive(Insertable)]
#[table_name="events"]
struct EventToCreate {
  name: String,
  description: Option<String>,
  url_slug: String,
  created_by: i32
}

#[derive(Insertable)]
#[table_name="topic_events"]
struct TopicEventToCreate {
  event_id: i32,
  topic_id: i32
}

#[post("/api/events", data = "<request_body>")]
pub fn create(request_body: Json<CreateEventRequestBody>) -> Json<Event> {
  let request: CreateEventRequestBody = request_body.into_inner();

  let evt_to_create = EventToCreate {
    name: request.name,
    description: request.description,
    url_slug: request.url_slug,
    created_by: 1
  };

  let evt_insert_res: Vec<Event> = diesel::insert_into(events::table)
    .values(evt_to_create)
    .load::<Event>(&crate::establish_connection())
    .expect("Could not insert event");
  
  let evt: Event = evt_insert_res.into_iter().nth(0).unwrap();

  let topic_events: Vec<TopicEventToCreate> = request.topics.iter().map(|topic_id| TopicEventToCreate {
    topic_id: *topic_id,
    event_id: evt.event_id
  }).collect::<Vec<TopicEventToCreate>>();

  diesel::insert_into(topic_events::table)
    .values(topic_events)
    .execute(&crate::establish_connection())
    .expect("Could not insert topic events");

  Json(evt)
}

#[get("/api/topics/<topic_id>/events?<perspective>")]
pub fn list(topic_id: i32, perspective: Option<String>) -> Json<Vec<EventWithPerspectives>> {
  let perspective_ids_str = perspective.unwrap_or(String::default());
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

    if !events_with_perspectives.get(&event_id).is_some() {
      events_with_perspectives.insert(event_id, EventWithPerspectives {
        event: event,
        perspectives: HashMap::new()
      });
    }

    let perspective_event_option: Option<PerspectiveEvent> = event_tuple.2;
    if perspective_event_option.is_some() {
      let persp_evt = perspective_event_option.unwrap();
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
