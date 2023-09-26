pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod db;
pub(crate) mod error;
pub(crate) mod http;
pub(crate) mod task;

use crate::http::{APIAuthToken, APITaskResponse, User};
use crate::command::echo::Echo;
use crate::error::Error;
use crate::http::{bad_request, forbidden, not_found, unauthorized, validate_credentials};

#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::serde::json::Json;
use rocket::State;
use rocket_basicauth::BasicAuth;
use std::env;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

#[get("/tasks/<name>/reset", format = "json")]
fn reset_task(
    name: &str,
    auth: Option<BasicAuth>,
    db: &State<db::MemDB>,
) -> Result<Json<APITaskResponse>, Error> {
    match validate_credentials(auth, db.get_user()?) {
        Err(_err) => Err(Error::Forbidden("Access forbidden.".to_string())),
        Ok(_ok) => {
            let duration = Duration::new(60, 0);
            db.rearm_task(name, duration)?;
            let task = db.get_task(name)?;
            Ok(Json(APITaskResponse::from_task(name, task)))
        }
    }
}

#[get("/tasks/<name>", format = "json")]
fn get_task(
    name: &str,
    auth: Option<BasicAuth>,
    db: &State<db::MemDB>,
) -> Result<Json<APITaskResponse>, Error> {
    match validate_credentials(auth, db.get_user()?) {
        Err(_err) => Err(Error::Forbidden("Access forbidden.".to_string())),
        Ok(_ok) => {
            let task = db.get_task(name)?;
            Ok(Json(APITaskResponse::from_task(name, task)))
        }
    }
}

#[get("/tasks", format = "json")]
fn list_tasks(
    auth: Option<BasicAuth>,
    db: &State<db::MemDB>,
) -> Result<Json<Vec<String>>, Error> {
    match validate_credentials(auth, db.get_user()?) {
        Err(_err) => Err(Error::Forbidden("Access forbidden.".to_string())),
        Ok(_ok) => {
            Ok(Json( db.list_tasks() ))
        }
    }
}

#[post("/login", format = "json", data = "<user>")]
fn login(
    user: Json<User>,
    db: &State<db::MemDB>,
) -> Result<Json<APIAuthToken>, Error> {
    // TODO: Issue a token used for authentication later on.
    // For now this is only used to validate the user and password.
    let cfg_user: User = db.get_user()?;
    if user.username == cfg_user.username && user.password == cfg_user.password {
        println!("login succeeded.");
        return Ok(Json(APIAuthToken{ token: "ok".to_string() }))
    }
    println!("login failed.");
    Err(Error::Forbidden("Access forbidden.".to_string()))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        let program: String = env::args().next().unwrap();
        eprintln!("Usage:");
        eprintln!("  {} ./clockwork.toml", program);
        exit(2);
    }

    let config: config::Config = match config::load_config(args[0].to_string()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2);
        }
    };

    println!("{}", config.task.name);

    let db = db::MemDB::new(
        config.auth.username,
        config.auth.password,
    );

    db.set_task(
        config.task.name,
        Arc::new(Echo {
            args: config.task.command.args,
        }),
        config.task.timeout,
    );

    const APIV1_PREFIX: &str = "/api/v1";

    let _rocket = rocket::build()
        .register(
            "/",
            catchers![bad_request, unauthorized, forbidden, not_found],
        )
        .mount(APIV1_PREFIX, routes![login])
        .mount(APIV1_PREFIX, routes![list_tasks])
        .mount(APIV1_PREFIX, routes![get_task])
        .mount(APIV1_PREFIX, routes![reset_task])
        .mount("/", FileServer::from(relative!("www/dist/static")))
        .manage(db)
        .launch()
        .await?;

    Ok(())
}
