use {
    super::{question::Question, quiz::Quiz},
    chrono::NaiveDateTime,
    diesel::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::quizzes_questions_answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Quiz, foreign_key = quiz_id))]
#[diesel(belongs_to(Question, foreign_key = question_id))]
#[diesel(primary_key(quiz_id, question_id))]
pub struct QuizAnswer {
    pub quiz_id: Uuid,
    pub question_id: Uuid,
    pub json_question_answer: serde_json::Value,
    pub create_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::quizzes_questions_answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewQuizAnswer<'a> {
    pub quiz_id: &'a Uuid,
    pub question_id: &'a Uuid,
    pub json_question_answer: &'a serde_json::Value,
}
