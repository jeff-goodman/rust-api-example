-- Your SQL goes here

CREATE TABLE users (
  id varchar(36) PRIMARY KEY,
  email varchar(256) UNIQUE NOT NULL,
  name varchar(256) DEFAULT NULL,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('users');

-- Seed data
INSERT INTO users (id, email, name)
  SELECT 
    gen_random_uuid(),
    'example' || generate_series || '@example.com',
    'Name ' || generate_series
    FROM generate_series(1, 10);