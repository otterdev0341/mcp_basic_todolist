// @generated automatically by Diesel CLI.

diesel::table! {
    todolist (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Text,
        is_done -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
