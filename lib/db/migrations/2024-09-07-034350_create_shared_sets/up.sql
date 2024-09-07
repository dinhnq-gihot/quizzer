-- Your SQL goes here
-- Set_Shared Table
CREATE TABLE shared_sets (
    set_id UUID NOT NULL REFERENCES sets(id),
    user_id UUID NOT NULL REFERENCES users(id),
    PRIMARY KEY (set_id, user_id)
);