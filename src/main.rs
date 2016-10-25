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

use std::path::Path;
use std::time::Instant;
use std::sync::Arc;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use pencil::{Request, PencilResult, Pencil};
use pencil::method::Get;

use util::DurationExt;
use config::Config;
use dao::{Dao, PostDao};
use util::JsonResponse;

mod model;
mod util;
mod dao;
mod config;
mod errors;
mod api;

lazy_static! {
    pub static ref APP: App = App::new(Some("config.json"));
}

pub struct App {
    pub conn_pool: r2d2::Pool<PostgresConnectionManager>,
    pub config: Config,
    pub pencil: Pencil,
}

impl App {
    pub fn new<P>(config_file: Option<P>) -> App
        where P: AsRef<Path>
    {
        let start = Instant::now();
        let config = Config::new(config_file);
        let manager = PostgresConnectionManager::new(config.db_string.as_str(), TlsMode::None)
            .unwrap();
        let pool = r2d2::Pool::new(r2d2::Config::default(), manager).unwrap();
        info!("Set up connection pool");

        let mut pencil = Pencil::new("/");
        let post_module = api::PostApi::get_module();
        pencil.register_module(post_module);

        info!("Initializing took {:?} ms", start.elapsed().millis());
        App {
            conn_pool: pool,
            config: config,
            pencil: pencil,
        }
    }

    pub fn run(self) {
        info!("Starting on {}:{}", self.config.host, self.config.port);
        self.pencil.run((self.config.host.as_str(), self.config.port))
    }

    pub fn drop_db(&self) -> errors::Result<()> {
        let conn = try!(self.conn_pool.get());
        util::execute_sql_file("drop_db.sql", &conn)
    }

    pub fn create_db(&self) -> errors::Result<()> {
        let conn = try!(self.conn_pool.get());
        util::execute_sql_file("create_db.sql", &conn)
    }
}

fn main() {
    let app = App::new(None::<&str>);
    app.run();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::UTC;

    use super::*;
    use super::dao::{self, Dao};
    use super::model::{Tag, User, Post};

    lazy_static! {
        static ref APP: App = {
            let app = App::new(Some("config_test.json"));
            app.drop_db().unwrap();
            app.create_db().unwrap();
            app
        };
    }

    #[test]
    fn test_inserts() {
        APP.drop_db().unwrap();
        APP.create_db().unwrap();

        let tags = vec![Tag {
                            name: String::from("test"),
                            id: 0,
                        },
                        Tag {
                            name: String::from("work"),
                            id: 0,
                        },
                        Tag {
                            name: String::from("other tag"),
                            id: 0,
                        }];

        let posts = vec![
            Post::new(0, "Post 1".into(), "Content 1".into(), 
                      UTC::now(), 1, vec![tags[0].clone(), tags[1].clone() ]),
            Post::new(0, "Post 2".into(), "Content 2".into(),
                      UTC::now(), 1, vec![tags[1].clone(), tags[2].clone()]),
            Post::new(0, "Post 3".into(), "Content 3".into(),
                      UTC::now(), 2, vec![tags[0].clone(), tags[2].clone()]),
        ];

        let mut user1 = User {
            name: String::from("martin"),
            pw_hash: String::from("test"),
            posts: vec![posts[0].clone(), posts[1].clone()],
            id: 0,
        };

        let mut user2 = User {
            name: "user2".into(),
            pw_hash: "test".into(),
            posts: vec![posts[2].clone()],
            id: 0,
        };

        let conn = Arc::new(APP.conn_pool.get().unwrap());
        let user_dao = dao::UserDao::new(conn.clone());
        user_dao.insert(&mut user1).unwrap();
        user_dao.insert(&mut user2).unwrap();

        assert_eq!(user1.id, 1);
        assert_eq!(user2.id, 2);
        let tag_dao = dao::TagDao::new(conn.clone());
        let tags = tag_dao.get_all().unwrap();
        assert_eq!(tags.len(), 3);

        let deleted = tag_dao.delete(tags[0].clone()).unwrap();

        let tags = tag_dao.get_all().unwrap();
        assert!(!tags.contains(&deleted));
    }
}
