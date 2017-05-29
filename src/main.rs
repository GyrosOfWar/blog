#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
#![allow(unknown_lints, needless_pass_by_value)]

#[allow(unused)]
#[macro_use(info, warn, debug, log)]
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
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate markdown;

extern crate rocket;
extern crate rocket_contrib;
extern crate ring_pwhash;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod auth;
mod model;
mod util;
mod config;
mod errors;
mod schema;
mod service;
mod db_util;

use std::env;
use std::path::{PathBuf, Path};

use rocket_contrib::Template;
use rocket::http::Cookie;
use rocket::request::{Form, FlashMessage};
use rocket::response::NamedFile;
use rocket::http::Session;
use rocket::response::{Redirect, Flash};

use db_util::Connection;
use model::{User, CreateUserRequest, CreatePostRequest, LoginRequest};
use errors::Result;
use service::user;

#[error(404)]
fn catch_404(_: &rocket::Request) -> Template {
    Template::render("404", &hashmap! {"parent" => "base"})
}

#[allow(unused_variables)]
#[get("/post/<name>/<id>")]
fn show_post_long(id: i32, name: String, conn: Connection) -> Result<Option<Template>> {
    show_post(id, conn)
}

#[get("/post/<id>")]
fn show_post(id: i32, conn: Connection) -> Result<Option<Template>> {
    let post = service::post::find_one(id, &conn)?;
    Ok(post.map(|p| {
                    let context = json! ({
            "parent": "base",
            "post": p.to_json()
        });
                    Template::render("show_post", &context)
                }))
}

#[get("/user/<id>")]
fn show_user(id: i32, conn: Connection) -> Result<Option<Template>> {
    match service::user::find_one(id, &*conn)? {
        Some(user) => {
            let posts = service::post::find_page(id, 0, 20, &*conn)?
                .map(|p| p.to_json());
            let context = json!({
                "parent": "base",
                "posts": posts,
                "user": user
            });

            Ok(Some(Template::render("show_user", &context)))
        }
        None => Ok(None),
    }
}

#[get("/login")]
fn login(flash: Option<FlashMessage>) -> Template {
    let mut context = hashmap! { "parent" => "base".to_string() };
    if let Some(msg) = flash {
        context.insert("flash", msg.msg().to_string());
    }
    Template::render("login", &context)
}

#[post("/login", data = "<data>")]
fn do_login(mut session: Session, data: Form<LoginRequest>, conn: Connection) -> Flash<Redirect> {
    let form = data.into_inner();
    if let Ok(Some(user)) = user::find_by_name(&form.name, &conn) {
        if user.verify_password(&form.password) {
            session.set(Cookie::new("user_id", user.id.to_string()));
            Flash::success(Redirect::to("/"), "Successfully logged in.")
        } else {
            Flash::error(Redirect::to("/login"), "Invalid username/password.")
        }
    } else {
        Flash::error(Redirect::to("/login"), "Invalid username/password.")
    }
}

#[post("/logout")]
fn do_logout(mut session: Session, user: User) -> Flash<Redirect> {
    let cookie = Cookie::new("user_id", user.id.to_string());
    session.remove(cookie);

    Flash::success(Redirect::to("/"), "You were logged out.")
}

#[post("/register", data = "<form>")]
fn new_user(form: Form<CreateUserRequest>, conn: Connection) -> Result<Flash<Redirect>> {
    let request = form.into_inner();
    service::user::create_user(request, &conn)?;

    Ok(Flash::success(Redirect::to("/"), "User created!"))
}

#[get("/")]
fn index(user: Option<User>, flash: Option<FlashMessage>) -> Template {
    let mut context = hashmap!{ "parent" => serde_json::to_value("base").unwrap(), "title" => serde_json::to_value("Blog").unwrap() };
    if let Some(user) = user {
        context.insert("user", serde_json::to_value(user).unwrap());
    }
    if let Some(msg) = flash {
        context.insert("flash", serde_json::to_value(msg.msg()).unwrap());
    }
    Template::render("index", &context)
}

#[get("/static/<file..>")]
fn serve_static_file(file: PathBuf) -> Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).map_err(From::from)
}

#[get("/post/new")]
fn post_editor(user: User) -> Template {
    let context = hashmap! { 
        "parent" => "base".to_string(),
        "user_id" => user.id.to_string(),
    };
    Template::render("write_post", &context)
}

#[post("/post/new", data = "<data>")]
fn create_post(data: Form<CreatePostRequest>, conn: Connection) -> Result<Flash<Redirect>> {
    let mut data = data.into_inner();
    data.convert_markdown();
    service::post::insert_post(data, &conn);
    Ok(Flash::success(Redirect::to("/"), "Post created!"))
}

fn main() {
    config::configure_logger();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(db_util::init_pool(&database_url))
        .attach(Template::fairing())
        .mount("/",
               routes![show_post, show_post_long, show_user, new_user, login, index, create_post,
                       do_login, serve_static_file, do_logout, post_editor])
        .catch(errors![catch_404])
        .launch();
}
