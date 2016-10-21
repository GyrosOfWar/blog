use std::time::Duration;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use errors::Result;
use dao::Connection;

/// Provides some additional conversions for Duration types.
pub trait DurationExt {
    /// Returns the whole duration in seconds, including the nano-second
    /// precision.
    fn seconds(&self) -> f64;

    /// Returns the whole duration in milliseconds, including
    /// the nano-second precision.
    fn millis(&self) -> f64;

    /// Creates a time from nanoseconds. (since the Duration::new function only
    // takes nanoseconds as a u32, which can easily overflow)
    fn from_nanos(nanos: u64) -> Duration;
}

impl DurationExt for Duration {
    #[inline]
    fn seconds(&self) -> f64 {
        self.as_secs() as f64 + self.subsec_nanos() as f64 / 1e9
    }

    #[inline]
    fn millis(&self) -> f64 {
        self.as_secs() as f64 * 1000.0 + (self.subsec_nanos() as f64 / 1e6)
    }

    #[inline]
    fn from_nanos(nanos: u64) -> Duration {
        if nanos > 1_000_000_000 {
            let seconds = nanos / 1_000_000_000;
            let nanos = nanos as u64 - (seconds as u64 * 1_000_000_000);
            Duration::new(seconds, nanos as u32)
        } else {
            Duration::new(0, nanos as u32)
        }
    }
}

pub fn execute_sql_file<P>(path: P, connection: &Connection) -> Result<()> 
    where P: AsRef<Path>
{
    let mut file = try!(File::open(&path));
    let mut text = String::new();
    try!(file.read_to_string(&mut text));
    info!("Executing SQL script {}", path.as_ref().display());
    for statement in text.split(';') {
        if statement.len() == 0 {
            continue;
        }
        let statement = statement.trim();
        debug!("Executing statement {}", statement);
        try!(connection.execute(statement, &[]));
    }
    info!("Finished executing SQL!");
    Ok(())
}