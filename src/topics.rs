use rocket_contrib::json::Json;

/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::*;

use crate::models::*;

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