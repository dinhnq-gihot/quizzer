-- Your SQL goes here
-- Create the role enum type
CREATE TYPE question_types AS ENUM ('TextFill', 'MultiChoice', 'CheckBoxes');

CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content TEXT NOT NULL,
    type question_types NOT NULL,
    set_id UUID NOT NULL REFERENCES sets(id),
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT FALSE
);