/* Import macros and others */
use diesel_derive_enum::DbEnum;
use diesel::sql_types::{ NotNull };
use chrono::NaiveDateTime;

/* For being able to serialize */
use serde::Serialize;
use serde::Deserialize;

#[derive(DbEnum, Debug)]
#[allow(non_camel_case_types)]
pub enum Source_type_enum {
  Primary,
  Secondary,
  Other
}
impl NotNull for Source_type_enum {}

#[allow(non_camel_case_types)]
#[derive(DbEnum, Debug, Serialize, Deserialize)]
pub enum Historicity_stance_enum {
  Fact,
  Fiction,
  Unknown,
  LeaningFact,
  LeaningFiction
}
impl NotNull for Historicity_stance_enum {}

#[derive(DbEnum, Debug)]
#[allow(non_camel_case_types)]
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
  pub name: String,
  pub description: Option<String>,
  pub url_slug: String,
  pub is_deleted: bool,
  pub is_latest: bool,
  pub original_event: i32,
  pub created_by: i32,
  pub created_date: NaiveDateTime
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
  pub perspective_event_id: i32,
  pub event_id: i32,
  pub perspective_id: i32,
  pub name: Option<String>,
  pub description: Option<String>,
  pub is_deleted: bool,
  pub historicity_stance: Historicity_stance_enum,
  pub relevance_stance: i32,
  pub created_by: i32,
  pub created_date: NaiveDateTime,
  pub is_latest: bool,
  pub original_perspective_event: i32
}