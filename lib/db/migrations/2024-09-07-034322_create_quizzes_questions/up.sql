-- Your SQL goes here
-- Quiz_Question Table
CREATE TABLE quizzes_questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quiz_id UUID NOT NULL REFERENCES quizzes(id),
    question_id UUID NOT NULL REFERENCES questions(id),
    question_content TEXT NOT NULL,
    type question_types NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT FALSE
);