use std::io::Cursor;
use thiserror::Error;
use rocket::response::Responder;
use rocket::serde::Serialize;
use rocket::{response, Request, Response};
use rocket::http::{ContentType, Status};

#[derive(Serialize)]
pub(crate) struct APIError {
    pub reason: String,
}

#[allow(dead_code)]
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    Unauthorized(String),
}


impl Error {
    fn get_http_status(&self) -> Status {
        match self {
            Error::Internal(_) => Status::InternalServerError,
            Error::NotFound(_) => Status::NotFound,
            Error::BadRequest(_) => Status::BadRequest,
            Error::Forbidden(_) => Status::Forbidden,
            Error::Unauthorized(_) => Status::Unauthorized,
            _ => Status::BadRequest,
        }
    }

}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let resp = serde_json::to_string(&APIError{
            reason: self.to_string()
        }).unwrap();

        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(resp.len(), Cursor::new(resp))
            .ok()
    }
}
