// @generated automatically by Diesel CLI.

diesel::table! {
    templates (id) {
        id -> Nullable<Integer>,
        name -> Text,
        path -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    templates,
    users,
);
