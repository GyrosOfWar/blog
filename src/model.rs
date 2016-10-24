use chrono::DateTime;
use chrono::UTC;
use dao::{UserDao, HasKey, Dao};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub id: i32,
}

impl HasKey<i32> for Tag {
    fn get_key(&self) -> i32 {
        self.id
    }
    fn set_key(&mut self, key: i32) {
        self.id = key
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub tags: Vec<Tag>,
    pub id: i32,
    pub created_on: DateTime<UTC>,
    pub owner_id: i32,
    owner: Option<User>,
}

impl HasKey<i32> for Post {
    fn get_key(&self) -> i32 {
        self.id
    }
    fn set_key(&mut self, key: i32) {
        self.id = key
    }
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

    pub fn get_owner<'a>(&mut self, user_dao: &UserDao) -> &'a User {
        // TODO cache the user in the struct
        unimplemented!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub pw_hash: String,
    pub posts: Vec<Post>,
    pub id: i32,
}

impl HasKey<i32> for User {
    fn get_key(&self) -> i32 {
        self.id
    }
    fn set_key(&mut self, key: i32) {
        self.id = key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_user() {
        let user = User {
            name: "martin".into(),
            pw_hash: "test".into(),
            posts: vec![],
            id: 1,
        };
        let json = serde_json::to_string(&user).unwrap();

        let user_de: User = serde_json::from_str(&json).unwrap();
        assert_eq!(user_de.name, String::from("martin"));
        assert_eq!(user_de.pw_hash, String::from(""));
        assert_eq!(user_de.id, 1);
    }
}
