use {
    super::{set::Set, user::User},
    chrono::NaiveDateTime,
    diesel::prelude::*,
    uuid::Uuid,
};

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::quizzes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Set, foreign_key = set_id))]
pub struct Quiz {
    pub id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
    pub user_id: Uuid,
    pub set_id: Uuid,
    pub public_or_not: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::quizzes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewQuiz<'a> {
    pub id: &'a Uuid,
    pub user_id: &'a Uuid,
    pub set_id: &'a Uuid,
    pub public_or_not: bool,
}