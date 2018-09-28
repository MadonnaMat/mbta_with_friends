#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::NamedFile;

use std::path::{Path, PathBuf};

#[get("/<file..>", rank=1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("webapp/dist/static").join(file)).ok()
}

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
    NamedFile::open(Path::new("webapp/dist/index.html")).ok()
}

#[get("/<_page..>", rank=4)]
fn index_extra(_page: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("webapp/dist/index.html")).ok()
}


fn main() {
    rocket::ignite()
        .mount("/static", routes![files])
        .mount("/api", routes![other, other_other])
        .mount("/", routes![index, index_extra])
        .launch();
}
