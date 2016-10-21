use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::env;
use std::fs::File;
use std::path::Path;

use rustc_serialize::json::Json;

use dotenv;
use log;
use time;
use env_logger;

static LOGGER_INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

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
    LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
}

pub struct Config {
    pub db_string: String,
    pub host: String,
    pub port: u16
}

impl Config {
    // TODO eliminate .unwrap()
    pub fn new<P>(config_file: Option<P>) -> Config 
        where P: AsRef<Path>
    {
        if !LOGGER_INITIALIZED.load(Ordering::SeqCst) {
            configure_logger();
        }
        let mut file = match config_file { 
            Some(p) => File::open(p).unwrap(),
            None => File::open("config.json").unwrap()
        };
        let json = Json::from_reader(&mut file).unwrap();

        Config {
            db_string: json.find_path(&["database", "url"]).unwrap().as_string().unwrap().into(),
            host: json.find_path(&["web", "host"]).unwrap().as_string().unwrap().into(),
            port: json.find_path(&["web", "port"]).unwrap().as_u64().unwrap() as u16
        }
    }
}