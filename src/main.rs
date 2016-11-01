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

use model::Post;

// 10 MB
const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

// TODO use bodyparser, json_response crates
fn get_post(req: &mut Request) -> IronResult<Response> {
    use schema::posts::dsl::*;

    unimplemented!()
    // let post_id: i32 = itry!(req.extensions.get::<Router>().unwrap().find("id").unwrap().parse());
    // let conn = req.db_conn();
    // let result: Vec<Post> = itry!(posts.filter(id.eq(post_id))
    //     .limit(1)
    //     .load::<Post>(&*conn));
    // if result.len() == 0 {
    //     let response: JsonResponse<String, _> =
    //         JsonResponse::Error(format!("No post found with ID {}", post_id));
    //     let response = itry!(serde_json::to_string(&response));
    //     Ok(Response::with(response))
    // } else {
    //     let response: JsonResponse<_, errors::Error> = JsonResponse::Result(result);
    //     let response = itry!(serde_json::to_string(&response));
    //     Ok(Response::with(response))
    // }
}

fn _add_post(req: &mut Request) -> errors::Result<Post> {
    use schema::posts;
    unimplemented!()
    // let conn = req.db_conn();
    // let post_request = try!(req.get::<bodyparser::Struct<CreatePostRequest>>()).unwrap();
    // post_request.content = api::markdown_to_html(&post_request.content);
    // let result: Vec<Post> = try!(diesel::insert(&post_request)
    //     .into(posts::table)
    //     .get_results(&*conn));
    // Ok(result[0].clone())
}

fn add_post(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
    // let result = JsonResponse::from_result(_add_post(req));
    // let response = itry!(serde_json::to_string(&result));
    // Ok(Response::with(response))
}

fn main() {
    dotenv::dotenv().unwrap();
    let api_router = router!(
        get_post: get "/user/:user_id/post/:post_id" => get_post,
        add_post: post "/user/:user_id/post" => add_post
    );

    let mut mount = Mount::new();
    mount.mount("/api", api_router);

    let db_url = env::var("DATABASE_URL").unwrap();
    let diesel_middleware = DieselMiddleware::new(&db_url).unwrap();
    let mut chain = Chain::new(mount);
    chain.link_before(diesel_middleware);
    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    let iron = Iron::new(chain);
    iron.http("localhost:5000").unwrap();
}