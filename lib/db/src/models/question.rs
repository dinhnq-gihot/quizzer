use {
    crate::{models::set::Set, schema::sql_types::QuestionTypes},
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
#[diesel(sql_type = QuestionTypes)]
pub enum QuestionType {
    TextFill,
    MultiChoice,
    CheckBoxes,
}

impl Default for QuestionType {
    fn default() -> Self {
        Self::TextFill
    }
}

impl ToSql<QuestionTypes, Pg> for QuestionType {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            QuestionType::TextFill => out.write_all(b"TextFill")?,
            QuestionType::MultiChoice => out.write_all(b"MultiChoice")?,
            QuestionType::CheckBoxes => out.write_all(b"CheckBoxes")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<QuestionTypes, Pg> for QuestionType {
    fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"TextFill" => Ok(QuestionType::TextFill),
            b"MultiChoice" => Ok(QuestionType::MultiChoice),
            b"CheckBoxes" => Ok(QuestionType::CheckBoxes),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Set, foreign_key = set_id))]
pub struct Question {
    pub id: Uuid,
    pub content: String,
    #[diesel(column_name = "type_")]
    pub r#type: QuestionType,
    pub set_id: Uuid,
    pub create_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewQuestion<'a> {
    pub id: &'a Uuid,
    pub content: &'a str,
    #[diesel(column_name = "type_")]
    pub r#type: &'a QuestionType,
    pub set_id: &'a Uuid,
}
