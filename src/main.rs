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

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};

use util::DurationExt;

mod model;
mod util;
mod dao;

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

pub struct Config {
    db_string: String,
}

impl Config {
    pub fn new(db_string: Option<String>) -> Config {
        // TODO
        Config { db_string: db_string.unwrap_or(String::from("postgres://martin:martin4817@localhost/blog")) }
    }
}

fn configure_logger() {
    dotenv::dotenv().unwrap();
    let format = |record: &log::LogRecord| {
        let cur_time = time::now();
        format!("{} [{}] {}: {}",
                cur_time.strftime("%Y-%m-%d %H:%M:%S:%f").unwrap(),
                record.level(),
                record.target(),
                record.args())
    };
    let mut builder = env_logger::LogBuilder::new();
    builder.format(format);
    builder.parse(&env::var("RUST_LOG").unwrap());
    builder.init().unwrap();
}

pub struct App {
    pub conn_pool: r2d2::Pool<PostgresConnectionManager>,
    pub config: Config,
}

impl App {
    pub fn new(db_string: Option<String>, init_logger: bool) -> App {
        let start = Instant::now();
        if init_logger {
            configure_logger();
        }
        info!("Set up logger");

        let config = Config::new(db_string);
        let manager = PostgresConnectionManager::new(config.db_string.as_str(), TlsMode::None)
            .unwrap();
        let pool = r2d2::Pool::new(r2d2::Config::default(), manager).unwrap();
        info!("Set up connection pool");
        info!("Initializing took {:?} ms", start.elapsed().millis());
        App {
            conn_pool: pool,
            config: config,
        }
    }

    pub fn drop_db(&self) -> errors::Result<()> {
        let conn = try!(self.conn_pool.get());
        execute_sql_file("drop_db.sql", &conn)
    }

    pub fn create_db(&self) -> errors::Result<()> {
        let conn = try!(self.conn_pool.get());
        execute_sql_file("create_db.sql", &conn)
    }
}

fn execute_sql_file<P>(path: P, connection: &dao::Connection) -> errors::Result<()> 
    where P: AsRef<Path>
{
    let mut file = try!(File::open(&path));
    let mut text = String::new();
    try!(file.read_to_string(&mut text));
    info!("Executing SQL script {}", path.as_ref().display());
    for statement in text.split(';') {
        let statement = statement.trim();
        debug!("Executing statement {}", statement);
        try!(connection.execute(statement, &[]));
    }
    info!("Finished executing SQL!");
    Ok(())
}

fn main() {
    let app = App::new(None, true);
}

#[cfg(test)]
#[allow(dead_code, unused)]
mod tests {
    use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};

    use super::*;
    use super::dao::{self, Dao};
    use super::model::Tag;

    lazy_static! {
        static ref APP: App = {
            let app = App::new(Some(String::from("postgres://martin:martin4817@localhost/blog_test")), true);
            app.drop_db().unwrap();
            app.create_db().unwrap();
            app
        };
    }

    fn insert_post(connection: &dao::Connection) {
        super::execute_sql_file("insert_post.sql", connection).unwrap();
    }
    // TODO synchronize access to the database to fix intermittent test failures

    #[test]
    fn test_tag_dao() {
        let conn = APP.conn_pool.get().unwrap();
        let tag_dao = dao::TagDao::new(conn);
        let name = String::from("test");
        let tag = Tag {
            name: name.clone(),
            id: 1,
        };
        tag_dao.insert_or_update(&tag).unwrap();

        let all = tag_dao.get_all().unwrap();
        assert_eq!(all[0].name, name);

        assert!(tag_dao.exists(&all[0].id).unwrap());

        let one = tag_dao.get_one(&all[0].id).unwrap();
        assert_eq!(one.name, name);
    }

    #[test]
    fn test_post_dao() {
        let connection = APP.conn_pool.get().unwrap();
        insert_post(&connection);
        let post_dao = dao::PostDao::new(connection);
        let post = post_dao.get_one(&1).unwrap();
        assert_eq!(post.id, 1);
        assert_eq!(post.content, String::from("This is the content of the test blog post"));
        assert_eq!(post.title, String::from("A test blog post"));
        assert!(post.tags.len() == 3);

        let all_posts = post_dao.get_all().unwrap();
        assert_eq!(all_posts.len(), 2);
    }
}