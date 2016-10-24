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
use pencil::Pencil;
use pencil::{Request, PencilResult, Response};
use pencil::method::Get;

use util::DurationExt;
use config::Config;
use dao::{Dao, PostDao};

mod model;
mod util;
mod dao;
mod config;

mod errors {
    use postgres;
    use r2d2;
    use std::io;
    use pencil;
    use std::error::Error as StdError;

    // TODO impl Serialze, Deserialize for Error
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Io(err: io::Error) {
                cause(err)
                from()
                description(err.description())
            }

            Postgres(err: postgres::error::Error) {
                cause(err)
                from()
                description(err.description())
            }

            ConnTimeout(err: r2d2::GetTimeout) {
                cause(err)
                from()
                description(err.description())
            }

            ExpectedResult {
                description("Expected a result, got none")
            }
        }
    }

    pub fn to_pencil_error<E>(err: E) -> pencil::PencilError
        where E: StdError
    {
        pencil::PencilError::PenUserError(pencil::UserError { desc: err.description().into() })
    }

    pub type Result<T> = ::std::result::Result<T, Error>;
}

lazy_static! {
    static ref APP: App = {
        let app = App::new(Some("config.json"));
        app.drop_db().unwrap();
        app.create_db().unwrap();
        app
    };
}

pub struct App {
    pub conn_pool: r2d2::Pool<PostgresConnectionManager>,
    pub config: Config,
    pub pencil: Pencil,
}

impl App {
    fn hello(_: &mut Request) -> PencilResult {
        Ok(Response::from("Hello world!"))
    }

    fn get_post(request: &mut Request) -> PencilResult {
        let conn = Arc::new(try!(APP.conn_pool.get().map_err(|e| errors::to_pencil_error(e))));
        let dao = PostDao::new(conn);
        let id = try!(request.view_args
            .get("id")
            .unwrap()
            .parse::<i32>()
            .map_err(|e| errors::to_pencil_error(e)));
        let post = dao.get_one(&id);
        unimplemented!()
        // pencil::jsonify(&post)
    }

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
        pencil.route("/", &[Get], "hello", App::hello);

        info!("Initializing took {:?} ms", start.elapsed().millis());
        App {
            conn_pool: pool,
            config: config,
            pencil: pencil,
        }
    }

    pub fn run(self) {
        info!("Startup..");
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
        // assert_eq!(tag_dao.get_one(&1).unwrap().name, "test".to_string());
        // assert_eq!(tag_dao.get_one(&2).unwrap().name, "work".to_string());
        // assert_eq!(tag_dao.get_one(&3).unwrap().name, "other tag".to_string());

    }
}
