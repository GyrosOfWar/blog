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

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate itertools;
extern crate chrono;
extern crate pencil;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate markdown;

mod model;
mod util;
mod dao;
mod config;
mod errors;
mod api;
mod app;

fn main() {
    // FIXME 
    // app::APP.run();
}