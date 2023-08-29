pub mod db;
pub mod task;
pub mod http;
pub mod error;
pub mod command;
pub mod config;

use error::CWError;
use task::Status;
use command::echo::Echo;
use http::{User, validate_credentials, bad_request,unauthorized,forbidden,not_found};

#[macro_use]
extern crate rocket;

use std::env;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;
use rocket_basicauth::BasicAuth;

#[derive(Serialize)]
pub struct JobStatus {
    pub name: String,
    pub status: Status,
    pub expires_in: u64,
    pub command: &'static str,
    pub result: String,
}

impl JobStatus {
    fn from_task(name: &str, task: task::Task) -> JobStatus {
        JobStatus {
            name: name.to_string(),
            status: task.status,
            expires_in: task.clone().expires_in(),
            command: task.clone().command.name(),
            result: task.clone().result,
        }
    }
}

#[get("/tasks/<name>/reset", format = "json")]
fn reset_task(auth: BasicAuth, name: &str, db: &State<db::MemDB>, user: &State<User>) -> Result<Json<JobStatus>, CWError> {
    validate_credentials(auth, user.inner().clone())?;

    let duration = Duration::new(60, 0);
    db.rearm_task(name, duration)?;
    let task = db.get_task(name)?;
    Ok(Json(JobStatus::from_task(name, task)))
}

#[get("/tasks/<name>", format = "json")]
fn get_task(auth: BasicAuth, name: &str, db: &State<db::MemDB>, user: &State<User>) -> Result<Json<JobStatus>, CWError> {
    validate_credentials(auth, user.inner().clone())?;

    let task = db.get_task(name)?;
    Ok(Json(JobStatus::from_task(name, task)))
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

    let db = db::MemDB::new();

    let config: config::Config;
    match config::load_config(args[0].to_string()) {
        Ok(cfg) => { config = cfg }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2);
        }
    }

    println!("{}", config.task.name);

    db.set_task(
        config.task.name,
        Arc::new(Echo{
            args: config.task.command.args,
        }),
        config.task.timeout,
    );

    let user = User {
        username: config.auth.username,
        password: config.auth.password,
    };

    let _rocket = rocket::build()
        .register("/", catchers![bad_request,unauthorized,forbidden,not_found])
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![get_task])
        .mount("/api", routes![reset_task])
        .manage(user)
        .manage(db)
        .launch()
        .await?;

    Ok(())
}