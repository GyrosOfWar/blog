#![feature(proc_macro)]

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

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
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

mod model;
mod util;
mod config;
mod errors;
mod api;
mod schema;

use std::env;

use diesel::prelude::*;
use iron::prelude::*;   
use iron::status;
use mount::Mount;
use router::Router;
use iron_diesel_middleware::{DieselMiddleware, DieselReqExt};

use model::Post;

fn get_post(req: &mut Request) -> IronResult<Response> {
    use schema::posts::dsl::*;

    // TODO get rid of unwraps;
    let post_id: i32 = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse().unwrap();

    let conn = req.db_conn();
    let result = itry!(posts.filter(id.eq(post_id))
        .limit(1)
        .load::<Post>(&*conn));

    Ok(Response::with((status::Ok, "Ok!")))
}

fn main() {
    dotenv::dotenv().unwrap();
    let api_router = router!(
        get_post: get "/post/:id" => get_post
    );

    let mut mount = Mount::new();
    mount.mount("/api", api_router);
    
    let db_url = env::var("DATABASE_URL").unwrap(); 
    let diesel_middleware = DieselMiddleware::new(&db_url).unwrap();
    let mut chain = Chain::new(mount);
    chain.link_before(diesel_middleware);
    
    let iron = Iron::new(chain);
    iron.http("localhost:5000").unwrap();
}