use serde::Serialize;
use serde::Deserialize;
use rocket_contrib::json::Json;

#[derive(Debug, Responder)]
pub enum RequestError {
  #[response(status = 400)]
  BadRequest(Json<RequestErrorExplanation>),
  #[response(status = 404)]
  NotFound(Json<RequestErrorExplanation>),
}

#[derive(Debug, Serialize)]
pub struct RequestErrorExplanation {
  pub error: &'static str
}
