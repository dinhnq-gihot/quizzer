-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS questions;

-- Drop the enum type
DROP TYPE IF EXISTS question_types;