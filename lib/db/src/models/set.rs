use {super::user::User, diesel::prelude::*};

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::sets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct Set {
    pub id: uuid::Uuid,
    pub public_or_not: bool,
    pub create_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub is_deleted: Option<bool>,
    pub user_id: uuid::Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::sets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSet<'a> {
    pub id: &'a uuid::Uuid,
    pub public_or_not: bool,
    pub user_id: &'a uuid::Uuid,
}
