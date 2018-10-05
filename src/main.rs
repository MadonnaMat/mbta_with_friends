#![allow(proc_macro_derive_resolution_fallback)]  //Remove when Diesel gets it Tish together
#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate dotenv;
extern crate rocket;
extern crate r2d2;
extern crate serde;
extern crate time;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

pub mod pg_pool;
pub mod models;
pub mod schema;
pub mod controllers;

use rocket::request::Request;
use rocket::response;
use rocket::response::NamedFile;
use rocket::response::Responder;
use rocket::response::Response;

use dotenv::dotenv;
use std::env;
use std::path::{Path, PathBuf};

pub struct NoCache(NamedFile);

impl<'r> Responder<'r> for NoCache {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.0.respond_to(req)?)
            .raw_header("Cache-control", "no-cache")
            .ok()
    }
}


const WEBAPP : &'static str = "mbta-with-friends/dist";

#[get("/", rank=3)]
fn index() -> Option<NamedFile> {
    let index = format!("{}/index.html", WEBAPP);

    NamedFile::open(Path::new(&index)).ok()
}

#[get("/service-worker.js", rank=3)]
fn service_worker() -> Option<NoCache> {
    let index = format!("{}/service-worker.js", WEBAPP);

    NamedFile::open(Path::new(&index)).ok().map(|mf| NoCache(mf) )
}

#[get("/<file..>", rank=4)]
fn index_extra(file: PathBuf) -> Option<NamedFile> {
    let file = NamedFile::open(Path::new(WEBAPP).join(file));
    
    if let Some(file) = file.ok() {
        Some(file)
    } else {
        let index = format!("{}/index.html", WEBAPP);

        NamedFile::open(Path::new(&index)).ok()
    }
}

fn routes() -> Vec<rocket::Route> {
    routes![
        controllers::users::all,
        controllers::users::all_bad,
        controllers::users::new_user,
        controllers::session::new_session,
        controllers::session::delete_session,
        controllers::config::config_logged_in,
        controllers::config::config,
    ]
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/api", routes())
        .mount("/", routes![index, service_worker, index_extra])
        .launch();
}
