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
    
#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, Serialize, Deserialize)]
#[has_many(posts)]
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
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub password: String,
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