// TODO use TOML file for configuration

pub struct Config {
    pub db_string: String,
    pub host: String,
    pub port: u16
}

impl Config {
    pub fn new(db_string: Option<String>) -> Config {
        // TODO
        Config { 
            db_string: db_string.unwrap_or(String::from("postgres://martin:martin4817@localhost/blog")),
            host: String::from("localhost"),
            port: 5000
        }
    }
}