use rocket_contrib::json::Json;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use crate::schema::*;
use serde::Serialize;
use crate::models::*;

#[derive(Serialize)]
pub struct GetMergeProposalsResponse {
  merge_proposals: Vec<MergeProposal>
}

#[get("/api/merge-proposals?<status>")]
pub fn get_merge_proposals(status: Option<Merge_proposal_status>) -> Json<GetMergeProposalsResponse> {
  let merge_proposals: Vec<MergeProposal> = merge_proposal_list_query(status, None)
    .load::<MergeProposal>(&crate::establish_connection())
    .expect("Error retrieving merge proposals");

  Json(GetMergeProposalsResponse {
    merge_proposals: merge_proposals
  })
}

#[get("/api/users/<user_id>/merge-proposals?<status>")]
pub fn get_user_merge_proposals(user_id: i32, status: Option<Merge_proposal_status>) -> Json<GetMergeProposalsResponse> {
  let merge_proposals: Vec<MergeProposal> = merge_proposal_list_query(status, Some(user_id)) 
    .load::<MergeProposal>(&crate::establish_connection())
    .expect("Error retrieving merge proposals");

  Json(GetMergeProposalsResponse {
    merge_proposals: merge_proposals
  })
}

fn merge_proposal_list_query(maybe_status: Option<Merge_proposal_status>, maybe_user_id: Option<i32>) -> BoxedSelectStatement<'static, (diesel::sql_types::Integer, diesel::sql_types::Integer, diesel::sql_types::Integer, Merge_proposal_statusMapping, diesel::sql_types::Integer, diesel::sql_types::Timestamp), merge_proposals::table, Pg> {
  let mut query = merge_proposals::table
    .select(merge_proposals::all_columns)
    .into_boxed();

  match maybe_status {
    Some(status_str) => {
      query = query.filter(merge_proposals::status.eq(status_str));
    }
    None => {}
  }

  match maybe_user_id {
    Some(user_id) => {
      query = query.filter(merge_proposals::created_by.eq(user_id))
    }
    None => {}
  }

  query
}