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
    owner_id INTEGER REFERENCES users (id) NOT NULL,
    tags VARCHAR[] NOT NULL DEFAULT '{}',
    published BOOLEAN NOT NULL DEFAULT FALSE
);