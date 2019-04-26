-- Your SQL goes here
CREATE TABLE users(
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  nickname VARCHAR NOT NULL,
  first_name VARCHAR NULL,
  last_name VARCHAR NULL
)