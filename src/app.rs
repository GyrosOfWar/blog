use std::path::Path;
use std::time::Instant;
use std::sync::Arc;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use pencil::{Request, PencilResult, Pencil};
use r2d2;
use pencil::method::Get;

use util::DurationExt;
use config::Config;
use dao::{self, Dao, PostDao};
use util::JsonResponse;
use errors;
use api;
use util;

lazy_static! {
    pub static ref APP: App = App::new(Some("config.json"));
}

pub fn get_connection() -> Arc<dao::Connection> {
    Arc::new(APP.conn_pool.get().unwrap())
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
        if config.debug {
            pencil.set_debug(true);
            pencil.set_log_level();
        }

        pencil.route("/api/user/", &[Get], "get_user", api::UserApi::get_user);

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
