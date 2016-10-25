use std::sync::Arc;

use r2d2;
use r2d2_postgres;
use postgres::types::ToSql;
use itertools::Itertools;

use errors::*;
use model::{Tag, Post, User};

pub type Connection = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

pub trait HasKey<K>
    where K: ToSql + Eq
{
    fn get_key(&self) -> K;
    fn set_key(&mut self, new_key: K);
}

pub trait Dao<T, K>
    where T: HasKey<K>,
          K: Eq + ToSql
{
    fn get_all(&self) -> Result<Vec<T>>;
    fn insert(&self, value: &mut T) -> Result<()>;
    fn update(&self, value: &T) -> Result<()>;
    fn get_one(&self, key: &K) -> Result<T>;
    fn exists(&self, key: &K) -> Result<bool>;
    fn delete(&self, value: T) -> Result<T>;

    fn delete_many<I>(&self, values: I) -> Result<()>
        where I: Iterator<Item = T>
    {
        for value in values {
            try!(self.delete(value));
        }
        Ok(())
    }

    fn insert_many<'a, I>(&self, values: I) -> Result<()>
        where I: Iterator<Item = &'a mut T>,
              T: 'a
    {
        for value in values {
            try!(self.insert(value));
        }
        Ok(())
    }
}

pub struct TagDao {
    conn: Arc<Connection>,
}

impl TagDao {
    pub fn new(conn: Arc<Connection>) -> TagDao {
        TagDao { conn: conn }
    }
}

impl Dao<Tag, i32> for TagDao {
    fn insert(&self, value: &mut Tag) -> Result<()> {
        let query = try!(self.conn
            .query("INSERT INTO tags (name) VALUES ($1) ON CONFLICT ON CONSTRAINT tags_name_key \
                    DO UPDATE SET name=tags.name RETURNING id",
                   &[&value.name]));
        let row = query.get(0);
        let key: i32 = row.get(0);
        value.set_key(key);
        Ok(())
    }

    fn update(&self, value: &Tag) -> Result<()> {
        try!(self.conn.execute("UPDATE tags SET name = $1 WHERE id = $2",
                               &[&value.name, &value.id]));
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
        Ok(query.iter()
            .map(|row| {
                Tag {
                    name: row.get(0),
                    id: row.get(1),
                }
            })
            .collect())
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

    fn delete(&self, tag: Tag) -> Result<Tag> {
        try!(self.conn.execute("DELETE FROM tags WHERE id = $1", &[&tag.id]));
        Ok(tag)
    }
}

pub struct PostDao {
    conn: Arc<Connection>,
}

impl PostDao {
    pub fn new(conn: Arc<Connection>) -> PostDao {
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

    pub fn get_posts_for_user(&self, user_id: i32) -> Result<Vec<Post>> {
        let mut posts = vec![];
        let sql = "
        SELECT posts.id, posts.title, posts.content, posts.created_on, tags.name, tags.id
        FROM users
            INNER JOIN users_posts ON users.id = users_posts.user_id
            INNER JOIN posts ON users_posts.post_id = posts.id
            FULL OUTER JOIN posts_tags ON posts.id = posts_tags.post_id
            FULL OUTER JOIN tags ON posts_tags.tag_id = tags.id
        WHERE users.id = $1";
        let query = try!(self.conn.query(sql, &[&user_id]));
        let iter = query.iter();
        for (_, mut group) in &iter.group_by(|row| row.get::<usize, i32>(0)) {
            let first = group.nth(0).unwrap();
            let mut post = Post::new(first.get(0),
                                     first.get(1),
                                     first.get(2),
                                     first.get(3),
                                     user_id,
                                     vec![]);
            for row in group {
                post.tags.push(Tag {
                    name: row.get(4),
                    id: row.get(5),
                })
            }
            info!("{:?}", post);
            posts.push(post);
        }
        Ok(posts)
    }
}

impl Dao<Post, i32> for PostDao {
    fn get_all(&self) -> Result<Vec<Post>> {
        let query = try!(self.conn
            .query("SELECT title, content, id, created_on, owner_id FROM posts",
                   &[]));
        let mut posts = vec![];
        for row in query.iter() {
            let id = row.get(2);
            let tags = try!(self.query_for_tags(id));
            posts.push(Post::new(id, row.get(0), row.get(1), row.get(3), row.get(4), tags))
        }
        Ok(posts)
    }

    fn insert(&self, value: &mut Post) -> Result<()> {
        let query = try!(self.conn
            .query("INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING id",
                   &[&value.title, &value.content]));
        let row = query.get(0);
        let post_id: i32 = row.get(0);
        let tag_dao = TagDao::new(self.conn.clone());
        try!(tag_dao.insert_many(value.tags.iter_mut()));
        value.set_key(post_id);
        Ok(())
    }

    fn update(&self, value: &Post) -> Result<()> {
        try!(self.conn.execute("UPDATE posts SET content = $1, title = $2 WHERE id = $3",
                               &[&value.content, &value.title, &value.id]));
        let tag_dao = TagDao::new(self.conn.clone());
        for tag in &value.tags {
            try!(tag_dao.update(&tag));
        }
        Ok(())
    }

    fn get_one(&self, key: &i32) -> Result<Post> {
        let query = try!(self.conn
            .query("SELECT title, content, id, created_on, owner_id FROM posts WHERE id = $1",
                   &[&key]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            let row = query.get(0);
            let tags = try!(self.query_for_tags(*key));
            Ok(Post::new(row.get(2),
                         row.get(0),
                         row.get(1),
                         row.get(3),
                         row.get(4),
                         tags))
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

    fn delete(&self, post: Post) -> Result<Post> {
        try!(self.conn.execute("DELETE FROM posts WHERE id = $1", &[&post.id]));
        Ok(post)
    }
}

pub struct UserDao {
    conn: Arc<Connection>,
}

impl UserDao {
    pub fn new(conn: Arc<Connection>) -> UserDao {
        UserDao { conn: conn }
    }
}

impl Dao<User, i32> for UserDao {
    fn get_all(&self) -> Result<Vec<User>> {
        let query = try!(self.conn.query("SELECT name, pw_hash, id FROM users", &[]));
        let mut users = vec![];
        let post_dao = PostDao::new(self.conn.clone());
        for row in query.iter() {
            let id = row.get(2);
            let posts = try!(post_dao.get_posts_for_user(id));
            users.push(User {
                name: row.get(0),
                pw_hash: row.get(1),
                id: id,
                posts: posts,
            });
        }
        Ok(users)
    }

    fn insert(&self, value: &mut User) -> Result<()> {
        let insert = try!(self.conn
            .query("INSERT INTO users (name, pw_hash) VALUES ($1, $2) RETURNING id",
                   &[&value.name, &value.pw_hash]));
        let row = insert.get(0);
        let user_id: i32 = row.get(0);
        value.set_key(user_id);
        let post_dao = PostDao::new(self.conn.clone());
        for post in &mut value.posts {
            try!(post_dao.insert(post));
        }

        Ok(())
    }

    fn update(&self, value: &User) -> Result<()> {
        try!(self.conn.execute("UPDATE users SET name = $1, pw_hash = $2 WHERE id = $3",
                               &[&value.name, &value.pw_hash, &value.id]));
        let post_dao = PostDao::new(self.conn.clone());
        for post in &value.posts {
            try!(post_dao.update(post));
        }

        Ok(())
    }

    fn get_one(&self, key: &i32) -> Result<User> {
        let query = try!(self.conn
            .query("SELECT name, pw_hash, id FROM users WHERE id = $1", &[&key]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            let post_dao = PostDao::new(self.conn.clone());
            let posts = try!(post_dao.get_posts_for_user(*key));
            let row = query.get(0);
            Ok(User {
                name: row.get(0),
                pw_hash: row.get(1),
                id: row.get(2),
                posts: posts,
            })
        }
    }

    fn exists(&self, key: &i32) -> Result<bool> {
        let query = try!(self.conn.query("SELECT COUNT(*) FROM users WHERE id = $1", &[key]));
        if query.is_empty() {
            Err(Error::ExpectedResult)
        } else {
            let row = query.get(0);
            let count: i64 = row.get(0);
            Ok(count > 0)
        }
    }

    fn delete(&self, user: User) -> Result<User> {
        try!(self.conn.execute("DELETE FROM users WHERE id = $1", &[&user.id]));
        Ok(user)
    }
}
