-- Your SQL goes here
CREATE TABLE sets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    public_or_not BOOLEAN NOT NULL,
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT FALSE,
    user_id UUID NOT NULL REFERENCES users(id)
);