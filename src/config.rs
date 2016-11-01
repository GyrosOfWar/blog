use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::env;
use std::fs::File;
use std::path::Path;

use serde_json;
use serde_json::Value;

use dotenv;
use log;
use time;
use env_logger;

static LOGGER_INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

pub fn configure_logger() {
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
    pub port: u16,
    pub debug: bool,
}

fn find_string(path: &str, value: &Value) -> String {
    let ptr = value.pointer(path);
    ptr.unwrap().as_str().unwrap().into()
}

fn find_u64(path: &str, value: &Value) -> u64 {
    let ptr = value.pointer(path);
    ptr.unwrap().as_u64().unwrap()
}

fn find_bool(path: &str, value: &Value) -> bool {
    let ptr = value.pointer(path);
    ptr.unwrap().as_bool().unwrap()
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
            None => File::open("config.json").unwrap(),
        };
        let json = serde_json::from_reader(&mut file).unwrap();
        Config {
            db_string: find_string("/database/url", &json),
            host: find_string("/web/host", &json),
            port: find_u64("/web/port", &json) as u16,
            debug: find_bool("/web/debug", &json),
        }
    }
}
