#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate r2d2;

use rocket::response::NamedFile;

use dotenv::dotenv;
use std::env;
use std::path::{Path, PathBuf};

mod pg_pool;
pub use self::pg_pool::DbConn;

const WEBAPP : &'static str = "mbta-with-friends/dist";

#[get("/", rank=2)]
fn other() -> &'static str {
    "Other"
}

#[get("/<page..>", rank=2)]
fn other_other(page: PathBuf) -> String {
    let page = page.to_str().unwrap_or("").to_string();
    format!("Hello from {}!", page)
}

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


fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/api", routes![other, other_other])
        .mount("/", routes![index, index_extra])
        .launch();
}
