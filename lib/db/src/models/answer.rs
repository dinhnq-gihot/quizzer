use {
    crate::{models::question::Question, schema::answers},
    chrono::NaiveDateTime,
    diesel::{prelude::*, Associations, Identifiable, Queryable},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Debug, Identifiable, AsChangeset, Selectable, Queryable, Associations, PartialEq, Clone,
)]
#[diesel(table_name = answers)]
#[diesel(belongs_to(Question, foreign_key = question_id))]
pub struct Answer {
    pub id: Uuid,
    pub content: String,
    pub is_correct: bool,
    pub question_id: Uuid,
    pub create_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAnswer<'a> {
    pub id: &'a Uuid,
    pub content: &'a str,
    pub is_correct: bool,
    pub question_id: &'a Uuid,
}
