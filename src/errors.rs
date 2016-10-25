use postgres;
use r2d2;
use std::io;
use pencil;
use std::error::Error as StdError;

use serde::{Serialize, Serializer};

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

pub fn to_pencil_error<E>(err: E) -> pencil::PencilError
    where E: StdError
{
    pencil::PencilError::PenUserError(pencil::UserError { desc: err.description().into() })
}

pub type Result<T> = ::std::result::Result<T, Error>;