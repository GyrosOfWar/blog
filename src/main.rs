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
mod api;
mod schema;
mod auth;
mod service;

use std::env;
use std::error;
use std::io::Read;

use diesel::prelude::*;
use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;
use iron_diesel_middleware::{DieselMiddleware, DieselReqExt};
use serde::Deserialize;

use model::Post;
use service::{UserService, CreateUserRequest};
use auth::UserCredentials;
use errors::*;

const SECRET: &'static [u8] = b"I LOVE FOOD";

// 100 MB
const MAX_BODY_LENGTH: usize = 1024 * 1024 * 100;

// TODO use bodyparser, json_response crates
fn get_post(req: &mut Request) -> IronResult<Response> {
    use schema::posts::dsl::*;
    unimplemented!()
}

fn add_post(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

fn read_json_body<T>(req: &mut Request) -> Result<T> 
    where T: Deserialize
{
    let mut body = String::new();
    try!(req.body.read_to_string(&mut body));
    debug!("Request body: {}", body);
    serde_json::from_str(&body).map_err(From::from)
}

fn make_token(req: &mut Request) -> IronResult<Response> {
    let conn = req.db_conn();
    let service = UserService::new(&*conn);
    match read_json_body::<UserCredentials>(req) {
        Ok(creds) => {
            let token_resp = service.make_token(&creds.user, &creds.password, SECRET);
            let json = itry!(serde_json::to_string(&token_resp));
            Ok(Response::with((status::Ok, json)))
        }
        Err(why) => {
            Ok(Response::with((status::BadRequest, format!("Error: {}", why))))
        }
    }
}

fn create_user(req: &mut Request) -> IronResult<Response> {
    let conn = req.db_conn();
    let service = UserService::new(&*conn);
    match read_json_body::<CreateUserRequest>(req) {
        Ok(req) => {
            let resp = service.create_user(req);
            let json = itry!(serde_json::to_string(&resp));
            Ok(Response::with(json))
        }
        Err(why) => {
            Ok(Response::with((status::BadRequest, format!("Error: {:?}", why))))
        }
    }
}

fn main() {
    config::configure_logger();
    let api_router = router!(
        get_post: get "/user/:user_id/post/:post_id" => get_post,
        add_post: post "/user/:user_id/post" => add_post,
        make_token: post "/token" => make_token,
        create_user: post "/user" => create_user
    );

    let mut mount = Mount::new();
    mount.mount("/api", api_router);

    let db_url = env::var("DATABASE_URL").unwrap();
    let diesel_middleware = DieselMiddleware::new(&db_url).unwrap();
    let mut chain = Chain::new(mount);
    chain.link_before(diesel_middleware);
    // chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    // chain.link_before(auth::JwtMiddleware::new(b"super secret"));
    let iron = Iron::new(chain);
    iron.http("localhost:5000").unwrap();
}