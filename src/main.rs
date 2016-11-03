#![feature(proc_macro, plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

extern crate env_logger;
extern crate dotenv;
extern crate time;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate itertools;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate markdown;

#[macro_use]
extern crate iron;
#[macro_use]
extern crate router;
extern crate mount;
extern crate iron_diesel_middleware;
extern crate bodyparser;
extern crate persistent;
extern crate jwt;
extern crate crypto;
extern crate pwhash;

mod model;
mod util;
mod config;
mod errors;
mod schema;
mod auth;
mod service;
mod controller;

use std::env;

use iron::prelude::*;
use router::Router;
use iron_diesel_middleware::DieselMiddleware;

use controller::{PostController, UserController};

pub const SECRET: &'static [u8] = b"I LOVE FOOD";

fn main() {
    config::configure_logger();
    let router = {
        let auth = auth::JwtMiddleware::new(SECRET);
        let mut router = Router::new();
        let mut chain = Chain::new(PostController::add_post);
        chain.link_before(auth);
        router.post("/api/user/:user_id/post", chain, "add_post");
        router.get("/api/user/:user_id/post/:post_id",
                   PostController::get_post,
                   "get_post");
        router.post("/api/token", UserController::make_jwt_token, "get_token");
        router.post("/api/user", UserController::create_user, "add_user");
        router
    };

    let db_url = env::var("DATABASE_URL").unwrap();
    let diesel_middleware = DieselMiddleware::new(&db_url).unwrap();
    let mut chain = Chain::new(router);
    chain.link_before(diesel_middleware);
    let iron = Iron::new(chain);
    iron.http("localhost:5000").unwrap();
}
