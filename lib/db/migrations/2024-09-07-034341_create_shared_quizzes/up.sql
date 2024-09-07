-- Your SQL goes here
-- Quiz_Shared Table
CREATE TABLE shared_quizzes (
    quiz_id UUID NOT NULL REFERENCES quizzes(id),
    user_id UUID NOT NULL REFERENCES users(id),
    PRIMARY KEY (quiz_id, user_id)
);