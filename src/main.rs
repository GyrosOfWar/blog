#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate env_logger;
extern crate dotenv;
extern crate time;
extern crate rustc_serialize;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate itertools;
extern crate chrono;
extern crate pencil;

use std::path::Path;
use std::time::Instant;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use pencil::Pencil;
use pencil::{Request, PencilResult, Response};
use pencil::method::Get;

use util::DurationExt;
use config::Config;

mod model;
mod util;
mod dao;
mod config;

mod errors {
    use postgres;
    use r2d2;
    use std::io;

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

    pub type Result<T> = ::std::result::Result<T, Error>;
}
pub struct App {
    pub conn_pool: r2d2::Pool<PostgresConnectionManager>,
    pub config: Config,
    pub pencil: Pencil
}

impl App {

    fn hello(_: &mut Request) -> PencilResult {
        Ok(Response::from("Hello world!"))
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
            pencil: pencil
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

    use super::*;
    use super::dao::{self, Dao};
    use super::model::Tag;

    lazy_static! {
        static ref APP: App = {
            let app = App::new(Some("config_test.json"));
            app.drop_db().unwrap();
            app.create_db().unwrap();
            app
        };
    }

    fn insert_post(connection: &dao::Connection) {
        super::util::execute_sql_file("insert_post.sql", connection).unwrap();
    }

    #[test]
    fn test_tag_dao() {
        let conn = Arc::new(APP.conn_pool.get().unwrap());
        let tag_dao = dao::TagDao::new(conn);
        let name = String::from("test");
        let tag = Tag {
            name: name.clone(),
            id: 1,
        };
        tag_dao.insert(&tag).unwrap();

        let all = tag_dao.get_all().unwrap();
        assert_eq!(all[0].name, name);

        assert!(tag_dao.exists(&all[0].id).unwrap());

        let one = tag_dao.get_one(&all[0].id).unwrap();
        assert_eq!(one.name, name);
    }

    #[test]
    fn test_post_dao() {
        let connection = Arc::new(APP.conn_pool.get().unwrap());
        insert_post(&connection);
        let post_dao = dao::PostDao::new(connection);
        let post = post_dao.get_one(&1).unwrap();
        assert_eq!(post.id, 1);
        assert_eq!(post.content, String::from("This is the content of the test blog post"));
        assert_eq!(post.title, String::from("A test blog post"));
        assert!(post.tags.len() == 3);

        let all_posts = post_dao.get_all().unwrap();
        assert_eq!(all_posts.len(), 3);

        let user_1_posts = post_dao.get_posts_for_user(1).unwrap();
        assert_eq!(user_1_posts.len(), 2);
        assert_eq!(user_1_posts[0].tags.len(), 2);
        assert_eq!(user_1_posts[1].tags.len(), 3);

        let user_2_posts = post_dao.get_posts_for_user(2).unwrap();
        assert_eq!(user_2_posts.len(), 1);
        assert_eq!(user_2_posts[0].tags.len(), 2);
    }
}