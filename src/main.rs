#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate dotenv;
extern crate rocket;
extern crate r2d2;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

pub mod pg_pool;
pub mod models;
pub mod schema;
pub mod controllers;

use rocket::response::NamedFile;

use dotenv::dotenv;
use std::env;
use std::path::{Path, PathBuf};

const WEBAPP : &'static str = "mbta-with-friends/dist";

#[get("/", rank=3)]
fn index() -> Option<NamedFile> {
    let index = format!("{}/index.html", WEBAPP);

    NamedFile::open(Path::new(&index)).ok()
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

fn user_routes() -> Vec<rocket::Route> {
    routes![
        controllers::users::all,
        controllers::users::new_user,
    ]
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/api", user_routes())
        .mount("/", routes![index, index_extra])
        .launch();
}
