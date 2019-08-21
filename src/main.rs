#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_derive;

use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket::FromForm;
use rocket::{get, post, routes};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    dotenv::dotenv().expect("Missing .env file");

    rocket::ignite()
        .mount(
            "/",
            routes![
                home,
                about,
                projects,
                resume,
                contact,
                download,
                files,
                force_reload,
                force
            ],
        )
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn home() -> Template {
    #[derive(Serialize)]
    struct Context {}

    let context = Context {};

    Template::render("views/home", &context)
}

#[get("/about")]
fn about() -> Template {
    #[derive(Serialize)]
    struct Context {}

    let context = Context {};

    Template::render("views/about", &context)
}

#[get("/projects")]
fn projects() -> Template {
    #[derive(Serialize)]
    struct Context {}

    let context = Context {};

    Template::render("views/projects", &context)
}

#[get("/resume")]
fn resume() -> Template {
    #[derive(Serialize)]
    struct Context {}

    let context = Context {};

    Template::render("views/resume", &context)
}

#[get("/contact")]
fn contact() -> Template {
    #[derive(Serialize)]
    struct Context {}

    let context = Context {};

    Template::render("views/contact", &context)
}

#[get("/files/<file..>")]
fn download(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files/").join(file)).ok()
}

#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[get("/reload")]
fn force_reload() -> Template {
    #[derive(Serialize)]
    struct Context {}

    let context = Context {};

    Template::render("views/reload", &context)
}

#[derive(FromForm)]
struct ForceReloadPayload {
    password: String,
}

#[post("/reload", data = "<payload>")]
fn force(payload: Form<ForceReloadPayload>) -> Redirect {
    let password = match env::var("ADMIN_PASSWORD") {
        Ok(database_url) => database_url,
        Err(_) => return Redirect::to("/"),
    };

    let reload_command = match env::var("RELOAD_COMMAND") {
        Ok(database_url) => database_url,
        Err(_) => return Redirect::to("/"),
    };

    if password == payload.password {
        Command::new("git").args(vec!["pull"]).output().unwrap();
        Command::new(reload_command).output().unwrap();
    }

    Redirect::to("/")
}
