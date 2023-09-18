-- Add down migration script here
ALTER TABLE todos
DROP COLUMN user_id;
