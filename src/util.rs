use std::time::Duration;
use std::error::Error as StdError;
use std::io::Read;

use iron::{IronResult, Response, status};
use iron::status::Status;
use serde_json;
use serde::Serialize;
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
    pub fn from_error(error: E) -> JsonResponse<(), E> {
        JsonResponse::Error(error)
    }

    pub fn from_result(result: ::std::result::Result<T, E>) -> JsonResponse<T, E> {
        match result {
            Ok(v) => JsonResponse::Result(v),
            Err(e) => JsonResponse::Error(e),
        }
    }

    pub fn into_iron_result(self, ok_status: Status, err_status: Status) -> IronResult<Response> {
        use self::JsonResponse::*;
        let json = serde_json::to_string(&self).unwrap();
        match self {
            Result(_) => Ok(Response::with((ok_status, json))),
            Error(_) => Ok(Response::with((err_status, json))),
        }
    }
}

fn convert_markdown_github(content: &str) -> Result<String> {
    let client = Client::new();
    let mime: Mime = "text/x-markdown".parse().unwrap();
    let mut res = try!(client.post("https://api.github.com/markdown/raw")
        .body(content)
        .header(ContentType(mime))
        .header(UserAgent("hyper/0.9.10".to_owned()))
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
