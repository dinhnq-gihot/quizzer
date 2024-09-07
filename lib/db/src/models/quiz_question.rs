use {
    super::{
        question::{Question, QuestionType},
        quiz::Quiz,
    },
    chrono::NaiveDateTime,
    diesel::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::quizzes_questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Quiz, foreign_key = quiz_id))]
#[diesel(belongs_to(Question, foreign_key = question_id))]
pub struct QuizQuestion {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub question_id: Uuid,
    pub question_content: String,
    #[diesel(column_name = "type_")]
    pub r#type: QuestionType,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::quizzes_questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewQuizQuestion<'a> {
    pub id: &'a Uuid,
    pub quiz_id: &'a Uuid,
    pub question_id: &'a Uuid,
    pub question_content: &'a str,
    #[diesel(column_name = "type_")]
    pub r#type: &'a QuestionType,
}
