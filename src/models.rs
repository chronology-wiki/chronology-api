/* Import macros and others */
use crate::schema::*;

/* For being able to serialize */
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Hero {
  pub id: i32,
   
  pub fantasy_name: String,
  pub real_name: Option<String>,
  pub spotted_photo: String,
  pub strength_level: i32,
}