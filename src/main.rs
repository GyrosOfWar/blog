#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
#![allow(needless_pass_by_value)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate time;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate maplit;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate markdown;

extern crate rocket;
extern crate rocket_contrib;
extern crate ring_pwhash;

mod model;
mod util;
mod config;
mod errors;
mod schema;
mod service;
mod db_util;

use std::env;
use std::collections::HashMap;
use std::path::{PathBuf, Path};

use rocket_contrib::{Template, JSON};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use serde_json::value::ToJson;
use ring_pwhash::scrypt;
use rocket::response::NamedFile;

use db_util::Connection;
use model::{CreateUserRequest, CreatePostRequest, LoginRequest};
use errors::Result;

pub const SECRET: &'static [u8] = b"I LOVE FOOD";

#[allow(unused_variables)]
#[get("/post/<name>/<id>")]
fn show_post_long(id: i32, name: String, conn: Connection) -> Result<Template> {
    show_post(id, conn)
}

#[get("/post/<id>")]
fn show_post(id: i32, conn: Connection) -> Result<Template> {
    let post = service::post::find_one(id, &*conn)?;
    if let Some(post) = post {
        let context = hashmap! {
            "parent" => "base".to_json()?,
            "post" => post.to_json()?
        };
        Ok(Template::render("show_post", &context))
    } else {
        Ok(Template::render("404", &hashmap! {"parent" => "base"} ))
    }
}

#[get("/user/<id>")]
fn show_user(id: i32, conn: Connection) -> Result<Template> {
    let user = service::user::find_one(id, &*conn)?;
    let posts = service::post::find_page(id, 0, 20, &*conn)?;
    let mut context = HashMap::new();
    context.insert("user", user.to_json()?);
    context.insert("posts", posts.to_json()?);
    Ok(Template::render("show_user", &context))
}

#[get("/login")]
fn login() -> Template {
    Template::render("login", &0)
}

#[post("/login", data = "<data>")]
fn do_login(data: Form<LoginRequest>, conn: Connection, cookies: &Cookies) -> Result<Template> {
    let form = data.into_inner();
    
    if let Some(user) = service::user::find_by_name(&form.name, &conn)? {
        if let Ok(true) = scrypt::scrypt_check(&form.password, &user.pw_hash) {
            cookies.add(Cookie::new("session_id", "1234"));
            return Ok(Template::render("index", &0));
        }
    }

    // TODO add errors
    Ok(Template::render("login", &0))
}

#[post("/register", data = "<form>")]
fn new_user(form: Form<CreateUserRequest>, conn: Connection) -> Result<Template> {
    let request = form.into_inner();
    service::user::create_user(request, &conn)?;

    Ok(Template::render("index", &0))
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &hashmap! {"parent" => "base", "title" => "Blog"} )
}

#[get("/static/<file..>")]
fn serve_static_file(file: PathBuf) -> Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).map_err(From::from)
}

#[post("/post/new", data = "<data>")]
fn create_post(mut data: JSON<CreatePostRequest>,
               conn: Connection)
               -> Result<Template> {
    data.convert_markdown();
    service::post::insert_post(data.0, &conn);
    Ok(Template::render("index", &0))
}

fn main() {
    config::configure_logger();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(db_util::init_pool(&database_url))
        .mount("/",
               routes![show_post, show_post_long, show_user, new_user, login, index, create_post,
                       do_login, serve_static_file])
        .launch()
}
