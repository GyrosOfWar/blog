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
use diesel::pg::PgConnection;
use iron::prelude::*;
use mount::Mount;

fn index(_: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    
    let api_router = router!(
        index: get "/" => index
    );

    let mut mount = Mount::new();
    mount.mount("/api", api_router);
    let iron = Iron::new(mount);
    iron.http("localhost:5000").unwrap();
}