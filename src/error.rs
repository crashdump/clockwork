use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::Serialize;
use rocket::{response, Request};

#[derive(Serialize)]
pub struct CWError {
    pub status: String,
    pub reason: String,
}

impl CWError {
    pub fn new(status: &str, reason: &str) -> CWError {
        CWError {
            status: status.to_string(),
            reason: reason.to_string(),
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for CWError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            _ => Status::BadRequest.respond_to(req),
        }
    }
}
