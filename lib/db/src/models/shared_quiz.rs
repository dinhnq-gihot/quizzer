use {
    super::{quiz::Quiz, user::User},
    diesel::prelude::*,
    uuid::Uuid,
};

#[derive(Debug, Identifiable, Selectable, Associations, Clone)]
#[diesel(table_name = crate::schema::shared_quizzes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Quiz, foreign_key = quiz_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(primary_key(quiz_id, user_id))]
pub struct SharedQuiz {
    pub quiz_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::shared_quizzes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSharedQuiz<'a> {
    pub quiz_id: &'a Uuid,
    pub user_id: &'a Uuid,
}
