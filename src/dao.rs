use r2d2;
use r2d2_postgres;
use postgres::types::ToSql;
use errors::*;

use model::{Tag, Post};

pub type Connection = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

pub trait Dao<T, K>
    where K: Eq + ToSql
{
    fn get_all(&self) -> Result<Vec<T>>;
    fn insert_or_update(&self, value: &T) -> Result<()>;
    fn get_one(&self, key: &K) -> Result<T>;
    fn value_exists(&self, value: &T) -> Result<bool>;
    fn key_exists(&self, key: &K) -> Result<bool>;
}

pub struct TagDao {
    conn: Connection,
}

impl TagDao {
    pub fn new(conn: Connection) -> TagDao {
        TagDao { conn: conn }
    }
}

impl Dao<Tag, i32> for TagDao {
    fn insert_or_update(&self, value: &Tag) -> Result<()> {
        try!(self.conn.execute("INSERT INTO tags (name) VALUES ($1) ON CONFLICT DO NOTHING",
                               &[&value.name]));
        Ok(())
    }

    fn get_one(&self, key: &i32) -> Result<Tag> {
        let query = try!(self.conn.query("SELECT name, id FROM tags WHERE id = $1", &[&key]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            let row = query.get(0);
            Ok(Tag {
                name: row.get(0),
                id: row.get(1),
            })
        }
    }

    fn get_all(&self) -> Result<Vec<Tag>> {
        let mut tags = vec![];
        for row in try!(self.conn.query("SELECT name, id FROM tags", &[])).iter() {
            let tag = Tag {
                name: row.get(0),
                id: row.get(1),
            };
            tags.push(tag);
        }

        Ok(tags)
    }

    fn value_exists(&self, value: &Tag) -> Result<bool> {
        let query = try!(self.conn.query("SELECT COUNT(*) FROM tags WHERE id = $1", &[&value.id]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            let row = query.get(0);
            let count: i64 = row.get(0);
            Ok(count > 0)
        }
    }

    fn key_exists(&self, key: &i32) -> Result<bool> {
        let query = try!(self.conn.query("SELECT COUNT(*) FROM tags WHERE id = $1", &[&key]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            let row = query.get(0);
            let count: i64 = row.get(0);
            Ok(count > 0)
        }
    }
}

pub struct PostDao {
    conn: Connection,
}

impl Dao<Post, i32> for PostDao {
    fn get_all(&self) -> Result<Vec<Post>> {
        unimplemented!()
    }

    fn insert_or_update(&self, value: &Post) -> Result<()> {
        unimplemented!()
    }

    fn get_one(&self, key: &i32) -> Result<Post> {
        let query = try!(self.conn.query("SELECT title, content, id FROM posts WHERE id = $1", &[&key]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            let row = query.get(0);
            let title: String = row.get(0);
            let content: String = row.get(1);
            let id: String = row.get(2);
            unimplemented!()
        }
    }

    fn value_exists(&self, value: &Post) -> Result<bool> {
        unimplemented!()
    }

    fn key_exists(&self, key: &i32) -> Result<bool> {
        unimplemented!()
    }

}