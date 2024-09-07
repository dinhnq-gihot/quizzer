-- Your SQL goes here
CREATE TABLE quizzes_questions_answers (
    quiz_id UUID NOT NULL REFERENCES quizzes(id),
    question_id UUID NOT NULL REFERENCES questions(id),
    json_question_answer JSONB NOT NULL,
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (quiz_id, question_id)
);