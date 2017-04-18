use chrono::{DateTime, UTC};
use schema::*;
use util;

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

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct User {
    pub name: String,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub pw_hash: String,
    pub id: i32,
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

#[derive(Deserialize, Debug, FromForm)]
pub struct CreateUserRequest {
    pub name: String,
    pub password: String,
    pub password_repeated: String,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}