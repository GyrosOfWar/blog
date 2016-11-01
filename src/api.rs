pub mod posts {
    use std::sync::Arc;
    use std::io::Read;

    use serde_json;
    use chrono::{UTC, DateTime};
    use hyper::Client;
    use hyper::header::{ContentType, UserAgent};
    use hyper::mime::Mime;
    use markdown;

    use util::JsonResponse;
    use errors::*;
    use model::*;
    use schema::posts;

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

    #[derive(Debug, Clone, Deserialize, Insertable)]
    #[table_name = "posts"]
    struct CreatePostRequest {
        pub title: String,
        pub content: String,
        pub tags: Vec<String>,
        pub owner_id: i32,
        #[serde(default = "UTC::now")]
        pub created_on: DateTime<UTC>,
    }


    // pub fn get_post(user_id: i32, post_id: i32) -> Result<Post> {}

    // pub fn add_post(request: CreatePostRequest) -> Result<Post> {}
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
