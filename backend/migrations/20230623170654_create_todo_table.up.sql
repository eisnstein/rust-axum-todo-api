-- Add up migration script here
CREATE TABLE IF NOT EXISTS todos (
  id SERIAL PRIMARY KEY,
  todo text NOT NULL,
  is_completed boolean NOT NULL DEFAULT false,
  updated_at timestamp NOT NULL DEFAULT NOW(),
  created_at timestamp NOT NULL DEFAULT NOW()
)
