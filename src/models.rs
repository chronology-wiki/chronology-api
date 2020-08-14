/* Import macros and others */
use diesel_derive_enum::DbEnum;
use diesel::sql_types::{ NotNull };
use chrono::NaiveDateTime;

/* For being able to serialize */
use serde::Serialize;

#[derive(DbEnum, Debug)]
pub enum Source_type_enum {
  Primary,
  Secondary,
  Other
}
impl NotNull for Source_type_enum {}

#[derive(DbEnum, Debug, Serialize)]
pub enum Historicity_stance_enum {
  Fact,
  Fiction,
  Unknown,
  LeaningFact,
  LeaningFiction
}
impl NotNull for Historicity_stance_enum {}

#[derive(DbEnum, Debug)]
pub enum User_organization_role_enum {
  Admin
}
impl NotNull for User_organization_role_enum {}


#[derive(Debug, Queryable, Serialize)]
pub struct Topic {
  pub topic_id: i32,
  pub name: String,
  pub url_slug: String,
  pub is_deleted: bool,
  pub original_topic: i32,
  pub created_by:i32,
  pub created_date: NaiveDateTime
}

#[derive(Debug, Queryable, Serialize)]
pub struct Event {
  pub event_id: i32,
  name: String,
  description: Option<String>,
  url_slug: String,
  is_deleted: bool,
  is_latest: bool,
  original_event: i32,
  created_by: i32,
  created_date: NaiveDateTime
}

#[derive(Debug, Queryable, Serialize)]
pub struct TopicEvent {
  event_id: i32,
  topic_id: i32
}

#[derive(Debug, Queryable, Serialize)]
pub struct Perspective {
  perspective_id: i32,
  url_slug: String,
  name: String,
  is_deleted: bool,
  is_latest: bool,
  original_perspective: i32,
  parent_perspective: Option<i32>,
  created_by: i32,
  created_date: NaiveDateTime
}

#[derive(Debug, Queryable, Serialize)]
pub struct PerspectiveEvent {
  perspective_event_id: i32,
  event_id: i32,
  perspective_id: i32,
  name: Option<String>,
  description: Option<String>,
  is_deleted: bool,
  historicity_stance: Historicity_stance_enum,
  created_by: i32,
  created_date: NaiveDateTime,
  is_latest: bool,
  original_perspective_event: i32
}