#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::NamedFile;

use std::path::{Path, PathBuf};

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

#[get("/<file..>", rank=4)]
fn index_extra(file: PathBuf) -> Option<NamedFile> {
    let file = NamedFile::open(Path::new("webapp/dist/").join(file));
    
    if let Some(file) = file.ok() {
        Some(file)
    } else {
        NamedFile::open(Path::new("webapp/dist/index.html")).ok()
    }
}


fn main() {
    rocket::ignite()
        .mount("/api", routes![other, other_other])
        .mount("/", routes![index, index_extra])
        .launch();
}
