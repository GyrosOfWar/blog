use chrono::DateTime;
use chrono::UTC;
use dao::UserDao;

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
    pub created_on: DateTime<UTC>,
    pub owner_id: i32,
    owner: Option<User>,
}

impl Post {
    pub fn new(id: i32,
               title: String,
               content: String,
               created_on: DateTime<UTC>,
               owner_id: i32,
               tags: Vec<Tag>)
               -> Post {
        Post {
            title: title,
            content: content,
            id: id,
            tags: tags,
            owner_id: owner_id,
            created_on: created_on,
            owner: None,
        }
    }

    pub fn get_owner(&self, user_dao: &UserDao) -> User {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub pw_hash: String,
    pub posts: Vec<Post>,
    pub id: i32,
}
