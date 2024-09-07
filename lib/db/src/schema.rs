// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "question_types"))]
    pub struct QuestionTypes;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_roles"))]
    pub struct UserRoles;
}

diesel::table! {
    answers (id) {
        id -> Uuid,
        content -> Text,
        is_correct -> Bool,
        question_id -> Uuid,
        create_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QuestionTypes;

    questions (id) {
        id -> Uuid,
        content -> Text,
        #[sql_name = "type"]
        type_ -> QuestionTypes,
        set_id -> Uuid,
        create_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    quizzes (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
        user_id -> Uuid,
        set_id -> Uuid,
        public_or_not -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QuestionTypes;

    quizzes_questions (id) {
        id -> Uuid,
        quiz_id -> Uuid,
        question_id -> Uuid,
        question_content -> Text,
        #[sql_name = "type"]
        type_ -> QuestionTypes,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    quizzes_questions_answers (quiz_id, question_id) {
        quiz_id -> Uuid,
        question_id -> Uuid,
        json_question_answer -> Jsonb,
        create_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    sets (id) {
        id -> Uuid,
        public_or_not -> Bool,
        create_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
        user_id -> Uuid,
    }
}

diesel::table! {
    shared_quizzes (quiz_id, user_id) {
        quiz_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    shared_sets (set_id, user_id) {
        set_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRoles;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        role -> UserRoles,
        create_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(questions -> sets (set_id));
diesel::joinable!(quizzes -> sets (set_id));
diesel::joinable!(quizzes -> users (user_id));
diesel::joinable!(quizzes_questions -> questions (question_id));
diesel::joinable!(quizzes_questions -> quizzes (quiz_id));
diesel::joinable!(quizzes_questions_answers -> questions (question_id));
diesel::joinable!(quizzes_questions_answers -> quizzes (quiz_id));
diesel::joinable!(sets -> users (user_id));
diesel::joinable!(shared_quizzes -> quizzes (quiz_id));
diesel::joinable!(shared_quizzes -> users (user_id));
diesel::joinable!(shared_sets -> sets (set_id));
diesel::joinable!(shared_sets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    questions,
    quizzes,
    quizzes_questions,
    quizzes_questions_answers,
    sets,
    shared_quizzes,
    shared_sets,
    users,
);
