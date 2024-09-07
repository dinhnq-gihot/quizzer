use {
    super::{set::Set, user::User},
    diesel::prelude::*,
    uuid::Uuid,
};

#[derive(Debug, Identifiable, Selectable, Associations, Clone)]
#[diesel(table_name = crate::schema::shared_sets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Set, foreign_key = set_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(primary_key(set_id, user_id))]
pub struct SharedSet {
    pub set_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::shared_sets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSharedSet<'a> {
    pub set_id: &'a Uuid,
    pub user_id: &'a Uuid,
}
