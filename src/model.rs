#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub tags: Vec<Tag>,
    pub id: i32,
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub pw_hash: String,
    pub posts: Vec<Post>,
    pub id: i32,
}
