use std::time::Duration;
use std::error::Error as StdError;
use std::io::Read;
use std::fmt;

use iron::{IronResult, Response, status};
use iron::status::Status;
use serde_json;
use serde::{Serialize, Serializer};
use hyper::Client;
use hyper::mime::Mime;
use hyper::header::{ContentType, UserAgent};

use errors::Result;

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

#[derive(Serialize)]
pub enum JsonResponse<T, E> {
    #[serde(rename="result")]
    Result(T),
    #[serde(rename="error")]
    Error(E),
}

impl<T, E> JsonResponse<T, E>
    where T: Serialize,
          E: StdError + Serialize
{
    pub fn into_iron_result(self, ok_status: Status, err_status: Status) -> IronResult<Response> {
        use self::JsonResponse::*;
        let json = serde_json::to_string(&self).unwrap();
        match self {
            Result(_) => Ok(Response::with((ok_status, json))),
            Error(_) => Ok(Response::with((err_status, json))),
        }
    }
}

impl<T, E> From<::std::result::Result<T, E>> for JsonResponse<T, E>
    where T: Serialize,
          E: StdError + Serialize
{
    fn from(result: ::std::result::Result<T, E>) -> JsonResponse<T, E> {
        match result {
            Ok(v) => JsonResponse::Result(v),
            Err(e) => JsonResponse::Error(e),
        }
    }
}

impl<T, E> Into<IronResult<Response>> for JsonResponse<T, E>
    where T: Serialize,
          E: StdError + Serialize
{
    fn into(self) -> IronResult<Response> {
        use self::JsonResponse::*;
        let json = serde_json::to_string(&self).unwrap();
        match self {
            Result(_) => Ok(Response::with((status::Ok, json))),
            Error(_) => Ok(Response::with((status::BadRequest, json))),
        }
    }
}

fn convert_markdown_github(content: &str) -> Result<String> {
    let client = Client::new();
    let mime: Mime = "text/x-markdown".parse().unwrap();
    let mut res = try!(client.post("https://api.github.com/markdown/raw")
        .body(content)
        .header(ContentType(mime))
        .header(UserAgent("hyper/0.9.11".to_owned()))
        .send());
    let mut text = String::new();
    try!(res.read_to_string(&mut text));
    Ok(text)
}

fn convert_markdown_fallback(content: &str) -> String {
    ::markdown::to_html(content)
}

pub fn markdown_to_html(input: &str) -> String {
    match convert_markdown_github(input) {
        Ok(res) => res,
        Err(why) => {
            warn!("Error when converting Markdown with Github API: {}", why);
            convert_markdown_fallback(input)
        }
    }
}

pub struct Page<T> {
    data: Vec<T>,
    current_page: i64,
    num_pages: i64,
    page_size: i64,
}

impl<T> Page<T> {
    pub fn new(data: Vec<T>, current_page: i64, num_pages: i64, page_size: i64) -> Page<T> {
        Page {
            data: data,
            current_page: current_page,
            num_pages: num_pages,
            page_size: page_size,
        }
    }
}

impl<T> fmt::Debug for Page<T>
    where T: fmt::Debug
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Page")
            .field("data", &self.data)
            .field("current_page", &self.current_page)
            .field("num_pages", &self.num_pages)
            .field("page_size", &self.page_size)
            .finish()
    }
}

impl<T> Serialize for Page<T>
    where T: Serialize
{
    fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_map(Some(1)));
        try!(serializer.serialize_map_key(&mut state, "data"));
        try!(serializer.serialize_map_value(&mut state, &self.data));

        try!(serializer.serialize_map_key(&mut state, "current_page"));
        try!(serializer.serialize_map_value(&mut state, &self.current_page));

        try!(serializer.serialize_map_key(&mut state, "num_pages"));
        try!(serializer.serialize_map_value(&mut state, &self.num_pages));

        try!(serializer.serialize_map_key(&mut state, "page_size"));
        try!(serializer.serialize_map_value(&mut state, &self.page_size));

        serializer.serialize_map_end(state)
    }
}