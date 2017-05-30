use std::collections::HashMap;

use chrono::{DateTime, UTC};
use schema::*;
use util;
use rocket::request::{FromForm, FormItems};
use regex::Regex;
use serde_json::{self, Value};

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Serialize, Deserialize, AsChangeset)]
#[table_name = "posts"]
pub struct Post {
    pub title: String,
    pub content: String,
    pub id: i32,
    pub created_on: DateTime<UTC>,
    pub owner_id: i32,
    pub tags: Vec<String>,
    pub published: bool,
}

impl Post {
    pub fn to_json(&self) -> Value {
        let mut value = serde_json::to_value(self).unwrap();
        {
            let mut obj = value.as_object_mut().unwrap();
            obj.insert("created_on_short".to_string(),
                       Value::String(format!("{}", self.created_on.format("%Y-%m-%d"))));
        }
        value
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct User {
    pub name: String,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub pw_hash: String,
    pub id: i32,
}

impl User {
    pub fn verify_password(&self, cleartext_pw: &str) -> bool {
        use ring_pwhash::scrypt;

        scrypt::scrypt_check(cleartext_pw, &self.pw_hash).unwrap_or(false)
    }
}

#[derive(Debug, Clone, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub owner_id: i32,
    #[serde(default = "UTC::now")]
    pub created_on: DateTime<UTC>,
    pub published: bool,
}

impl CreatePostRequest {
    pub fn convert_markdown(&mut self) {
        self.content = util::markdown_to_html(&self.content);
    }
}

impl<'r> FromForm<'r> for CreatePostRequest {
    type Error = String;

    fn from_form_items(form_items: &mut FormItems<'r>) -> Result<CreatePostRequest, Self::Error> {
        lazy_static! {
            static ref TAGS_REGEX: Regex = Regex::new("\\s").unwrap();
        }
        const KEYS: &[&str] = &["title", "content", "tags", "owner_id", "published"];

        let mut items = HashMap::new();
        for (k, v) in form_items {
            let decoded = v.url_decode().map_err(|e| format!("{}", e))?;
            items.insert(k.as_str(), decoded);
        }

        for key in KEYS {
            if !items.contains_key(key) {
                return Err(format!("Missing form parameter: {}", key));
            }
        }

        let mut tags = vec![];
        for tag in TAGS_REGEX.split(&items["tags"]) {
            tags.push(tag.into());
        }

        let owner = items["owner_id"]
            .parse()
            .map_err(|e| format!("Failed to parse owner ID: {}", e))?;
        let published = items["published"].parse().unwrap_or(false);

        Ok(CreatePostRequest {
               title: items["title"].clone(),
               content: items["content"].clone(),
               tags: tags,
               owner_id: owner,
               created_on: UTC::now(),
               published: published,
           })
    }
}

#[derive(Serialize, Deserialize, FromForm, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub password: String,
    pub password_repeated: String,
}

#[derive(Serialize, Deserialize, FromForm, Debug)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}
