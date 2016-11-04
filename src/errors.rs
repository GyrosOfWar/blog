use std::num;

use r2d2;
use std::io;
use hyper;
use std::error::Error as StdError;
use diesel;
use serde_json;

use serde::{Serialize, Serializer};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
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

        Hyper(err: hyper::Error) {
            cause(err)
            from()
            description(err.description())
        }

        Diesel(err: diesel::result::Error) {
            cause(err)
            from()
            description(err.description())
        }

        SerdeJson(err: serde_json::Error) {
            cause(err)
            from()
            description(err.description())
        }

        InvalidCredentials {
            description("Invalid credentials")
        }

        CreateToken {
            description("Could not create token")
        }

        Parse(err: num::ParseIntError) {
            cause(err)
            from()
            description(err.description())
        }

        Other(payload: String) {
            description(payload)
        }
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_map(Some(1)));
        try!(serializer.serialize_map_key(&mut state, "description"));
        try!(serializer.serialize_map_value(&mut state, self.description()));
        serializer.serialize_map_end(state)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
