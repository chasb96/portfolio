#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_derive;

use rocket::response::NamedFile;
use rocket::{get, routes};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use std::path::{Path, PathBuf};

fn main() {
    rocket::ignite()
        .mount("/", routes![home, files])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn home() -> Template {
    #[derive(Serialize)]
    struct Context {}

    let context = Context {};

    Template::render("home", &context)
}

#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}
