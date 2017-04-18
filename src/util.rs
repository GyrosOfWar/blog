use std::time::Duration;
use std::error::Error as StdError;
use std::io::Read;
use std::fmt;

use reqwest::header::{ContentType, UserAgent};
use reqwest::mime::Mime;
use serde_json;
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use errors::Result;
use reqwest;

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

fn convert_markdown_github(content: &str) -> Result<String> {
    let client = reqwest::Client::new()?;
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

fn convert_markdown_plain(content: &str) -> String {
    ::markdown::to_html(content)
}

pub fn markdown_to_html(input: &str, mode: MarkdownMode) -> String {
    match mode {
        MarkdownMode::Github => {
            convert_markdown_github(input).unwrap_or(convert_markdown_plain(input))
        }
        MarkdownMode::Plain => convert_markdown_plain(input),
    }
}

pub enum MarkdownMode {
    Plain,
    Github,
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
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_key("data")?;
        map.serialize_value(&self.data)?;

        map.serialize_key("current_page")?;
        map.serialize_value(&self.current_page)?;

        map.serialize_key("num_pages")?;
        map.serialize_value(&self.num_pages)?;

        map.serialize_key("page_size")?;
        map.serialize_value(&self.page_size)?;

        map.end()
    }
}
