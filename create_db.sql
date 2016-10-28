CREATE TABLE IF NOT EXISTS tags (
    name VARCHAR NOT NULL UNIQUE,
    id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS users (
    name VARCHAR NOT NULL,
    pw_hash VARCHAR NOT NULL,
    id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS posts (
    title VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    id SERIAL PRIMARY KEY,
    created_on TIMESTAMP WITH TIME ZONE NOT NULL,
    owner_id INTEGER REFERENCES users (id) NOT NULL
);

CREATE TABLE IF NOT EXISTS posts_tags (
    post_id INTEGER REFERENCES posts (id),
    tag_id INTEGER REFERENCES tags (id)
);

CREATE TABLE IF NOT EXISTS users_posts (
    user_id INTEGER REFERENCES users (id),
    post_id INTEGER REFERENCES posts (id)
);