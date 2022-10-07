-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    comment TEXT,
    added TIMESTAMP
)