pub mod db;
pub mod task;
pub mod error;
pub mod command;
pub mod config;

use error::CWError;
use task::Status;
use command::echo::Echo;

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
fn reset_task(name: &str, db: &State<db::MemDB>) -> Result<Json<JobStatus>, CWError> {
    let duration = Duration::new(60, 0);
    db.rearm_task(name, duration)?;
    let task = db.get_task(name)?;
    Ok(Json(JobStatus::from_task(name, task)))
}

#[get("/tasks/<name>", format = "json")]
fn get_task(name: &str, db: &State<db::MemDB>) -> Result<Json<JobStatus>, CWError> {
    let task = db.get_task(name)?;
    Ok(Json(JobStatus::from_task(name, task)))
}

#[catch(404)]
fn not_found() -> Json<CWError> {
    Json(CWError::new("error", "Resource was not found."))
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

    let _rocket = rocket::build()
        .register("/", catchers![not_found])
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![get_task])
        .mount("/api", routes![reset_task])
        .manage(db)
        .launch()
        .await?;

    Ok(())
}
