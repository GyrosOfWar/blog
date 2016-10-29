CREATE TABLE tags (
    name VARCHAR NOT NULL UNIQUE,
    id SERIAL PRIMARY KEY
);

CREATE TABLE users (
    name VARCHAR NOT NULL,
    pw_hash VARCHAR NOT NULL,
    id SERIAL PRIMARY KEY
);

CREATE TABLE posts (
    title VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    id SERIAL PRIMARY KEY,
    created_on TIMESTAMP WITH TIME ZONE NOT NULL,
    owner_id INTEGER REFERENCES users (id) NOT NULL
);

CREATE TABLE posts_tags (
    post_id INTEGER REFERENCES posts (id),
    tag_id INTEGER REFERENCES tags (id),
    PRIMARY KEY (post_id, tag_id)
);

CREATE TABLE users_posts (
    user_id INTEGER REFERENCES users (id),
    post_id INTEGER REFERENCES posts (id),
    PRIMARY KEY (user_id, post_id)
);