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
    fn exists(&self, key: &K) -> Result<bool>;
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
        let query = try!(self.conn.query("SELECT name, id FROM tags", &[]));
        Ok(query.iter().map(|row| Tag { name: row.get(0), id: row.get(1) }).collect())
    }

    fn exists(&self, key: &i32) -> Result<bool> {
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

impl PostDao {
    pub fn new(conn: Connection) -> PostDao {
        PostDao { conn: conn }
    }

    fn query_for_tags(&self, id: i32) -> Result<Vec<Tag>> {
        let query = try!(self.conn
            .query("SELECT t.name, t.id FROM posts_tags p INNER JOIN tags t ON p.tag_id = t.id \
                    WHERE post_id = $1",
                   &[&id]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            Ok(query.iter()
                .map(|row| {
                    Tag {
                        name: row.get(0),
                        id: row.get(1),
                    }
                })
                .collect())
        }
    }
}

impl Dao<Post, i32> for PostDao {
    fn get_all(&self) -> Result<Vec<Post>> {
        let query = try!(self.conn.query("SELECT title, content, id FROM posts", &[]));
        let mut posts = vec![];
        for row in query.iter() {
            let id = row.get(2);
            let tags = try!(self.query_for_tags(id));
            posts.push(Post {
                title: row.get(0),
                content: row.get(1),
                id: id,
                tags: tags
            })
        }
        Ok(posts)
    }

    fn insert_or_update(&self, value: &Post) -> Result<()> {
        unimplemented!()
    }

    fn get_one(&self, key: &i32) -> Result<Post> {
        let query = try!(self.conn.query("SELECT title, content, id FROM posts WHERE id = $1",
                                         &[&key]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            let row = query.get(0);
            let tags = try!(self.query_for_tags(*key));

            Ok(Post {
                title: row.get(0),
                content: row.get(1),
                id: row.get(2),
                tags: tags,
            })
        }
    }

    fn exists(&self, key: &i32) -> Result<bool> {
        let query = try!(self.conn.query("SELECT COUNT(*) FROM posts WHERE id = $1", &[key]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            let row = query.get(0);
            let count: i64 = row.get(0);
            Ok(count > 0)
        }
    }
}