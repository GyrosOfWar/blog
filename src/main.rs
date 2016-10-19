#![recursion_limit = "1024"]

extern crate postgres;
#[macro_use]
extern crate error_chain;

use std::rc::Rc;

use postgres::Connection;
use postgres::types::ToSql;

use errors::*;

mod errors {
    error_chain! { }
}

pub trait Dao<T, K> 
    where K: Eq + ToSql
{
    fn get_all(&mut self) -> postgres::Result<Vec<T>>;
    fn insert_or_update(&mut self, value: T) -> postgres::Result<()>;
    fn get_one(&mut self, key: K) -> postgres::Result<T>;
    fn value_exists(&mut self, value: T) -> postgres::Result<bool>;
    fn key_exists(&mut self, key: K) -> postgres::Result<bool>;
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub id: u32,
}

pub struct TagDao {
    conn: Rc<Connection>,
}

impl Dao<Tag, u32> for TagDao {
    fn insert_or_update(&mut self, value: Tag) -> postgres::Result<()> {
        try!(self.conn.execute("INSERT INTO tags (name) VALUES ($1) ON CONFLICT DO NOTHING", &[&value.name]));
        Ok(())
    }

    fn get_one(&mut self, key: u32) -> postgres::Result<Tag> {
        let query = try!(self.conn.query("SELECT name, id FROM tags WHERE id = $1", &[&key]));
        if query.is_empty() {
            unimplemented!()
        } else {
            let row = query.get(0);
            Ok(Tag {
                name: row.get(0),
                id: row.get(1)
            })
        }
    }

    fn get_all(&mut self) -> postgres::Result<Vec<Tag>> {
        let mut tags = vec![];
        for row in try!(self.conn.query("SELECT name, id FROM tags", &[])).iter() {
            let tag = Tag {
                name: row.get(0),
                id: row.get(1)
            };
            tags.push(tag);
        }

        Ok(tags)
    }

    fn value_exists(&mut self, value: Tag) -> postgres::Result<bool> {
        // let query = try!(self.conn.query("SELECT "));
        unimplemented!()
    }

    fn key_exists(&mut self, key: u32) -> postgres::Result<bool> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_dao() {
        // let tag = Tag
    }
}