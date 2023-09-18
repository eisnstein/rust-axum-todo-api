-- Add up migration script here
ALTER TABLE todos
ADD COLUMN user_id integer;