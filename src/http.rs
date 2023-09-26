use crate::error::Error;
use crate::task;
use crate::task::TaskStatus;

use rocket_basicauth::BasicAuth;
use serde::{Deserialize, Serialize};

#[catch(400)]
pub(crate) fn bad_request() -> Error {
    Error::BadRequest("Bad request.".to_string())
}

#[catch(401)]
pub(crate) fn unauthorized() -> Error {
    Error::Unauthorized("Access unauthorized.".to_string())
}

#[catch(403)]
pub(crate) fn forbidden() -> Error {
    Error::Forbidden("Access forbidden.".to_string())
}

#[catch(404)]
pub(crate) fn not_found() -> Error {
    Error::NotFound("Resource was not found.".to_string())
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct User {
    pub username: String,
    pub password: String,
}

pub(crate) fn validate_credentials(auth: Option<BasicAuth>, user: User) -> Result<(), ()> {
    match auth {
        Some(value) => {
            if value.username == user.username && value.password == user.password {
                return Ok(());
            }
            if value.username != user.username {
                println!("unknown user '{}'", value.username);
                return Err(());
            }
            println!("invalid credentials for user '{}'", value.username);
            Err(())
        }
        None => {
            Err(())
        }
    }
}

#[derive(Serialize)]
pub(crate) struct APIAuthToken {
    pub token: String,
}

#[derive(Serialize)]
pub(crate) struct APITaskResponse {
    pub name: String,
    pub status: TaskStatus,
    pub expires_in: u64,
    pub command: &'static str,
    pub result: String,
}

impl APITaskResponse {
    pub(crate) fn from_task(name: &str, task: task::Task) -> APITaskResponse {
        APITaskResponse {
            name: name.to_string(),
            status: task.status,
            expires_in: task.clone().expires_in(),
            command: task.clone().command.name(),
            result: task.clone().result,
        }
    }
}