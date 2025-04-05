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
