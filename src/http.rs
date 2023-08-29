use crate::error::CWError;
use rocket::serde::json::Json;
use rocket_basicauth::BasicAuth;

#[catch(400)]
pub(crate) fn bad_request() -> Json<CWError> {
    Json(CWError::new("error", "Bad request."))
}

#[catch(401)]
pub(crate) fn unauthorized() -> Json<CWError> {
    Json(CWError::new("error", "Access unauthorized."))
}

#[catch(403)]
pub(crate) fn forbidden() -> Json<CWError> {
    Json(CWError::new("error", "Access forbidden."))
}

#[catch(404)]
pub(crate) fn not_found() -> Json<CWError> {
    Json(CWError::new("error", "Resource was not found."))
}

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub fn validate_credentials(auth: BasicAuth, user: User) -> Result<(), CWError> {
    if auth.username != user.username {
        println!("unknown user '{}'", auth.username);
        return Err(CWError::new("error", "invalid username or password."))
    }
    
    match auth.username == user.username && auth.password == user.password {
        true => Ok(()),
        false => {
            println!("invalid credentials for user '{}'", auth.username);
            return Err(CWError::new("error", "invalid username or password."))
        },
    }
}
