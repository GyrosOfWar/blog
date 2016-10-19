#![recursion_limit = "1024"]

#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate time;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

use std::env;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};

mod model;
mod dao;

mod errors {
    use postgres;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Postgres(err: postgres::error::Error) {
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
    pub fn new() -> Config {
        // TODO
        Config { db_string: String::from("postgres://martin:martin4817@localhost/blog") }
    }
}

fn configure_logger() {
    dotenv::dotenv().unwrap();
    let format = |record: &log::LogRecord| {
        let cur_time = time::now();
        format!("{:02}:{:02}:{:02} [{}] {}: {}",
                cur_time.tm_hour,
                cur_time.tm_min,
                cur_time.tm_sec,
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
    pub fn new() -> App {
        configure_logger();
        info!("Set up logger");

        let config = Config::new();
        let manager = PostgresConnectionManager::new(config.db_string.as_str(), TlsMode::None)
            .unwrap();
        let pool = r2d2::Pool::new(r2d2::Config::default(), manager).unwrap();
        info!("Set up connection pool");
        App {
            conn_pool: pool,
            config: config,
        }
    }
}

fn main() {
    let app = App::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::dao::{self, Dao};
    use super::model::Tag;

    #[test]
    fn test_tag_dao() {
        let app = App::new();
        let conn = app.conn_pool.get().unwrap();
        let tag_dao = dao::TagDao::new(conn);
        let name = String::from("test");
        let tag = Tag {
            name: name.clone(),
            id: 1,
        };
        tag_dao.insert_or_update(&tag).unwrap();

        let all = tag_dao.get_all().unwrap();
        assert_eq!(all[0].name, name);

        assert!(tag_dao.value_exists(&all[0]).unwrap());
        assert!(tag_dao.key_exists(&all[0].id).unwrap());

        let one = tag_dao.get_one(&all[0].id).unwrap();
        assert_eq!(one.name, name);
    }
}