-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS users;

-- Drop the enum type
DROP TYPE IF EXISTS user_roles;