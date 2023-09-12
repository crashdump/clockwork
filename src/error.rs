use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::Serialize;
use rocket::{response, Request};

#[derive(Serialize)]
pub(crate) struct CWError {
    pub status: String,
    pub reason: String,
}

impl CWError {
    pub(crate) fn new(status: &str, reason: &str) -> CWError {
        CWError {
            status: status.to_string(),
            reason: reason.to_string(),
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for CWError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Status::BadRequest.respond_to(req)
    }
}
