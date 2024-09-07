-- Your SQL goes here
-- Quiz Table
CREATE TABLE quizzes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT FALSE,
    user_id UUID NOT NULL REFERENCES users(id),
    set_id UUID NOT NULL REFERENCES sets(id),
    public_or_not BOOLEAN NOT NULL
);