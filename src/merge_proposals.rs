use rocket_contrib::json::Json;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::result::QueryResult;
use diesel::result::Error;
use diesel::result::DatabaseErrorKind;
use crate::schema::*;
use serde::Serialize;
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct CreateMergeProposalRequestBody {
  source_perspective_id: i32,
  target_perspective_id: i32
}

#[derive(Insertable)]
#[table_name = "merge_proposals"]
struct MergeProposalToCreate {
  source_perspective_id: i32,
  target_perspective_id: i32,
  status: Merge_proposal_status,
  created_by: i32
}

#[derive(Debug, Responder)]
pub enum RequestError {
  #[response(status = 400)]
  BadRequest(Json<RequestErrorExplanation>),
  #[response(status = 404)]
  NotFound(Json<RequestErrorExplanation>),
}

#[derive(Debug, Serialize)]
pub struct RequestErrorExplanation {
  error: &'static str
}

#[post("/api/merge-proposals", data = "<request_body>")]
pub fn create_merge_proposal(request_body: Json<CreateMergeProposalRequestBody>) -> Result<Json<MergeProposal>, RequestError> {
  let proposal = MergeProposalToCreate {
    source_perspective_id: request_body.source_perspective_id,
    target_perspective_id: request_body.target_perspective_id,
    status: Merge_proposal_status::Draft,
    created_by: 1
  };

  let db_res: QueryResult<Vec<MergeProposal>> = diesel::insert_into(merge_proposals::table)
    .values(proposal)
    .load::<MergeProposal>(&crate::establish_connection());

  match db_res {
    Ok(merge_proposals) => Ok(Json(merge_proposals.into_iter().nth(0).unwrap())),
    Err(Error::DatabaseError(error_kind, _info)) => {
      match error_kind {
        DatabaseErrorKind::UniqueViolation => Err(RequestError::BadRequest(Json(RequestErrorExplanation {
          error: "Merge proposal already exists"
        }))),
        DatabaseErrorKind::ForeignKeyViolation => Err(RequestError::NotFound(Json(RequestErrorExplanation {
          error: "One or more perspective ids do not exist"
        }))),
        _ => panic!("unknown error")
      }
    },
    _ => panic!("Unknown error")
  }
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