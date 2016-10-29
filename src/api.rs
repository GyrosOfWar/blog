use std::sync::Arc;
use std::io::Read;

use serde_json;
use chrono::UTC;
use hyper::Client;
use hyper::header::{ContentType, UserAgent};
use hyper::mime::Mime;
use markdown;

use util::JsonResponse;
use errors::*;
use model::*;

fn convert_markdown_github(content: &str) -> Result<String> {
    let mut client = Client::new();
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
    markdown::to_html(content)
}

fn markdown_to_html(input: &str) -> String {
    match convert_markdown_github(input) {
        Ok(res) => res,
        Err(why) => {
            warn!("Error when converting Markdown with Github API: {}", why);
            convert_markdown_fallback(input)
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct CreatePostRequest {
    title: String,
    content: String,
    tags: Vec<String>,
    owner_id: i32,
}

impl CreatePostRequest {
    fn to_post(self) -> Result<Post> {
        let html_content = markdown_to_html(&self.content);
        Ok(Post {
            title: self.title,
            content: html_content,
            id: 0,
            tags: self.tags.into_iter().map(|t| Tag { name: t, id: 0 }).collect(),
            created_on: UTC::now(),
            owner_id: self.owner_id,
        })
    }
}

pub struct UserApi;

impl UserApi {
}

mod tests {
    #[test]
    fn test_markdown_conversion() {
        let input = "# Heading";
        let expected = "Heading</h1>";
        let converted_github = super::convert_markdown_github(input).unwrap();
        info!("{}", converted_github);
        assert!(converted_github.contains(expected))
    }
}
