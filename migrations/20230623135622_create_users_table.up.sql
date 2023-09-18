-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  email text UNIQUE NOT NULL,
  password_hash text NOT NULL,
  updated_at timestamp NOT NULL DEFAULT NOW(),
  created_at timestamp NOT NULL DEFAULT NOW()
);
