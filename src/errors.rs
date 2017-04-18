use std::num;

use r2d2;
use std::io;
use std::error::Error as StdError;
use diesel;
use serde_json;

use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;

error_chain! {
    foreign_links {
        Io(io::Error);
        DbTimeout(r2d2::GetTimeout);
        Reqwest(::reqwest::Error);
        Diesel(diesel::result::Error);
        SerdeJson(::serde_json::Error);
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_key("description")?;
        map.serialize_value(self.description())?;
        map.end()
    }
}
