use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::env;

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
