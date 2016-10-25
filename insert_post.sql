DELETE FROM users_posts;
DELETE FROM posts_tags;
DELETE FROM tags;
DELETE FROM posts;
DELETE FROM users;

ALTER SEQUENCE posts_id_seq RESTART WITH 1;
ALTER SEQUENCE tags_id_seq RESTART WITH 1;
ALTER SEQUENCE users_id_seq RESTART WITH 1;

INSERT INTO users (name, pw_hash, id) VALUES 
  ('martin', 'todo', 1),
  ('user2', 'todo', 2);

INSERT INTO posts (title, content, id, created_on, owner_id) VALUES
  ('A test blog post', 'This is the content of the test blog post', 1, now(), 1),
  ('Another test blog post!', 'Some more #content #value', 2, now(), 1),
  ('Post from User 2', 'Content of the post!', 3, now(), 2);

INSERT INTO tags (name, id) VALUES ('content', 1), ('test', 2), ('testing', 3), ('stuff', 4), ('user2', 5), ('junk', 6);

INSERT INTO posts_tags (post_id, tag_id) VALUES 
(1, 1), (1, 2), (1, 3),
  (2, 3), (2, 4),
  (3, 5), (3, 6);

INSERT INTO users_posts (user_id, post_id) VALUES (1, 1), (1, 2), (2, 2);