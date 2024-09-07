use {
    crate::schema::sql_types::UserRoles,
    diesel::{
        deserialize::{FromSql, FromSqlRow},
        expression::AsExpression,
        pg::Pg,
        prelude::*,
        serialize::{IsNull, ToSql},
    },
    serde::{Deserialize, Serialize},
    std::io::Write,
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow, PartialEq, Clone)]
#[diesel(sql_type = UserRoles)]
pub enum UserRole {
    Staff,
    User,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::User
    }
}

impl ToSql<UserRoles, Pg> for UserRole {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            UserRole::User => out.write_all(b"User")?,
            UserRole::Staff => out.write_all(b"Staff")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<UserRoles, Pg> for UserRole {
    fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"User" => Ok(UserRole::User),
            b"Staff" => Ok(UserRole::Staff),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub create_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub role: &'a UserRole,
}
