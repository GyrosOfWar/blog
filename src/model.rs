use chrono::{DateTime, UTC};
use schema::*;

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[belongs_to(User)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub id: i32,
    pub created_on: DateTime<UTC>,
    pub owner_id: i32,
    pub tags: Vec<String>,
}

impl Post {
    pub fn new(id: i32,
               title: String,
               content: String,
               created_on: DateTime<UTC>,
               owner_id: i32,
               tags: Vec<String>)
               -> Post {
        Post {
            title: title,
            content: content,
            id: id,
            tags: tags,
            owner_id: owner_id,
            created_on: created_on,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[has_many(posts)]
#[table_name = "users"]
pub struct User {
    pub name: String,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub pw_hash: String,
    // pub posts: Vec<Post>,
    pub id: i32,
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
