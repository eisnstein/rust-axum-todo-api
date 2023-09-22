-- Add up migration script here
ALTER TABLE users
ADD COLUMN is_admin boolean DEFAULT false;
