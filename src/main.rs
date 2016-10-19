extern crate postgres;

use postgres::Connection;
use postgres::types::ToSql;

pub trait Dao<T, K> 
    where K: Eq + ToSql
{
    fn get_all(&mut self) -> postgres::Result<Vec<T>>;
    fn insert_or_update(&mut self, value: T) -> postgres::Result<()>;
    fn get_one(&mut self, key: K) -> postgres::Result<T>;
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub id: u32,
}

pub struct TagDao {
    conn: Connection 
}

impl Dao<Tag, u32> for TagDao {
    fn insert_or_update(&mut self, value: Tag) -> postgres::Result<()> {
        try!(self.conn.execute("", &[]));
        Ok(())
    }

    fn get_one(&mut self, key: u32) -> postgres::Result<Tag> {
        unimplemented!()
    }

    fn get_all(&mut self) -> postgres::Result<Vec<Tag>> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct Post {
    pub title: String,
    pub text: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub pw_hash: String,
    pub posts: Vec<Post>,
}

fn main() {
    println!("Hello, world!");
}
