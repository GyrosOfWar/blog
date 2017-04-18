#![feature(plugin, custom_attribute)]
#![plugin(rocket)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate time;
#[macro_use]
extern crate error_chain;

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

mod model;
mod util;
mod config;
mod errors;
mod schema;
// mod auth;
// mod service;
// mod controller;

use std::env;

pub const SECRET: &'static [u8] = b"I LOVE FOOD";

fn main() {
    config::configure_logger();
    // TODO add PUT for editing posts
    // let router = {
    //     let auth = auth::JwtMiddleware::new(SECRET);
    //     let mut router = Router::new();
    //     let mut chain = Chain::new(PostController::add_post);
    //     chain.link_before(auth.clone());
    //     router.post("/api/user/:user_id/post", chain, "add_post");
    //     let mut chain = Chain::new(UserController::get_user);
    //     chain.link_before(auth.clone());
    //     router.get("/api/user/:user_id", chain, "get_user");
    //     let mut chain = Chain::new(UserController::edit_post);
    //     chain.link_before(auth);
    //     router.put("/api/user/:user_id/post/:post_id", chain, "edit_post");
    //     router.get("/api/user/:user_id/post/:post_id",
    //                PostController::get_post,
    //                "get_post");
    //     router.get("/api/user/:user_id/post",
    //                PostController::get_posts,
    //                "get_posts");
    //     router.post("/api/token", UserController::make_jwt_token, "get_token");
    //     router.post("/api/user", UserController::create_user, "add_user");
    //     router
    // };

    // let db_url = env::var("DATABASE_URL").unwrap();
    // let diesel_middleware = DieselMiddleware::new(&db_url).unwrap();
    // let mut chain = Chain::new(router);
    // chain.link_before(diesel_middleware);
    // let iron = Iron::new(chain);
    // iron.http("localhost:5000").unwrap();
}
