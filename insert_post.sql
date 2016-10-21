DELETE FROM users_posts;
DELETE FROM posts_tags;
DELETE FROM tags;
DELETE FROM posts;
DELETE FROM users;

ALTER SEQUENCE posts_id_seq RESTART WITH 1;
ALTER SEQUENCE tags_id_seq RESTART WITH 1;
ALTER SEQUENCE users_id_seq RESTART WITH 1;

INSERT INTO posts (title, content) VALUES
  ('A test blog post', 'This is the content of the test blog post'),
  ('Another test blog post!', 'Some more #content #value');

INSERT INTO tags (name) VALUES ('content'), ('test'), ('testing'), ('stuff') ON CONFLICT DO NOTHING;

INSERT INTO posts_tags (post_id, tag_id) VALUES (1, 1), (1, 2), (1, 3),
  (2, 3), (2, 4);

INSERT INTO users (name, pw_hash) VALUES ('martin', 'todo');

INSERT INTO users_posts (user_id, post_id) VALUES (1, 1), (1, 2)